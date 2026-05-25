use crate::component::register::Registry;
use crate::gate::entry_gate::EntryGate;
use crate::gate::exit_gate::ExitGate;
use crate::component::notify::Notify;
use crate::entity::permit::Permit;
use crate::entity::san::{Pollutant, SanSchema, SanSignal};
use crate::blocklist::Blocklist;
use crate::config::Config;
use tracing::{info, warn, error};

/// 通用 Gate 校验：重试 + SAN 扣减
///
/// 职责瘦身：只管"重试 + 扣分"，不再做阻塞决策。
/// 阻塞判定统一由 Blocklist::enforce() 在 run_once() 中执行。
///
/// 返回 true = 校验通过，false = 重试耗尽或 SAN 归零
/// 目前还存在解析带来的性能损耗，后续考虑优化
async fn run_gate<E: Pollutant + std::fmt::Debug>(
    permit: &mut Permit,
    config: &Config,
    gate_name: &str,
    mut operation: impl FnMut(&mut Permit) -> Result<(), Vec<E>>,
) -> bool {
    // 临时取出 SAN，避免 &mut permit 和 &mut permit.san 的借用冲突
    // Gate 校验不访问 san 字段，所以临时替换为空值是安全的
    let mut san = std::mem::replace(&mut permit.san, SanSchema::new(0));

    for attempt in 0..=config.max_retries {
        if attempt > 0 {
            info!(component = gate_name, attempt = attempt, max = config.max_retries, "正在重试...");
        }

        match operation(permit) {
            Ok(()) => {
                if attempt > 0 {
                    info!(component = gate_name, attempt = attempt, "重试成功");
                }
                permit.san = san; // 放回
                return true;
            }
            Err(errors) => {
                warn!(component = gate_name, attempt = attempt, "校验失败");
                for e in &errors {
                    let penalty = config.san_overrides.get(e.error_key()).copied()
                        .unwrap_or_else(|| e.weight());

                    // SAN 发出信号：归零则立即终止重试
                    if san.apply_penalty(penalty, e.error_key()) == SanSignal::Corrupted {
                        error!(component = gate_name, "SAN 归零信号触发，立即终止重试");
                        permit.san = san; // 放回
                        return false;
                    }
                }
            }
        }
    }

    // 重试耗尽
    permit.san = san; // 放回
    false
}

pub async fn run_once(mut payload: serde_json::Value) -> Result<serde_json::Value, String> {
    info!("=== 流水线启动（单次执行） ===");

    let config = Config::load().map_err(|e| format!("配置加载失败: {}", e))?;

    // ──── 1. Enroll（登记）：动态签发专属 Permit ────
    let mut permit = Registry::enroll_task();
    permit.payload = payload.to_string();
    info!(
        permit_id = permit.permit_id,
        status = %permit.permit_status,
        san = permit.san.current_san,
        "拿到凭证"
    );

    // ──── 2. Exit Gate（出门校验）：准备离开 agent，去调用外部工具 ────
    if !run_gate(&mut permit, &config, "Exit", |p| ExitGate::check_out(p)).await {
        Blocklist::enforce(&mut permit, &config).await
            .map_err(|signal| format!("Exit Gate 拦截: {}", signal))?;
        return Err(format!("Exit Gate 重试耗尽，permit_id: {}", permit.permit_id));
    }

    // ──── 3. Execute（执行）：转发上游 LLM ────
    let json_resp = execute_llm(&config, &mut payload).await?;
    permit.payload = json_resp.to_string();

    // ──── 4. Blocklist（阻塞名单审查）：LLM 返回后立即扫描危险内容 ────
    Blocklist::enforce(&mut permit, &config).await
        .map_err(|signal| format!("阻塞名单拦截: {}", signal))?;

    // ──── 5. Entry Gate（进门校验）：LLM 返回格式/结构安全检查 ────
    if !run_gate(&mut permit, &config, "Entry", |p| EntryGate::check_in(p)).await {
        Blocklist::enforce(&mut permit, &config).await
            .map_err(|signal| format!("Entry Gate 拦截: {}", signal))?;
        return Err(format!("Entry Gate 重试耗尽，permit_id: {}", permit.permit_id));
    }

    // ──── 6. Finish（完成）：标记生命周期结束 ────
    permit.finish_with_log();
    Notify::send("任务完成 (Done)", &format!(
        "任务完成，ID: {}，最终SAN: {}/{}",
        permit.permit_id, permit.san.current_san, permit.san.max_san
    )).await;

    info!(permit_id = permit.permit_id, "=== 流水线执行完毕 ===");
    Ok(json_resp)
}

/// 执行步骤：模型重定向 + 转发上游 LLM + 记录响应
/// 后续扩展点：多 Provider 支持、Streaming、工具调用等
async fn execute_llm(
    config: &Config,
    payload: &mut serde_json::Value,
) -> Result<serde_json::Value, String> {
    // 强制把模型名称替换成配置中的模型
    if let Some(obj) = payload.as_object_mut() {
        obj.insert("model".to_string(), serde_json::json!(config.model));
    }
    info!("已将请求模型重定向为: {}", config.model);

    // 转发请求到上游 LLM
    info!("正在将请求转发给上游 LLM (Groq)...");
    let client = reqwest::Client::new();
    let resp = client
        .post("https://api.groq.com/openai/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", config.groq_api_key))
        .json(&*payload)
        .send()
        .await
        .map_err(|e| format!("代理转发失败: {}", e))?;

    let status = resp.status();
    let json_resp: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| format!("解析上游响应失败: {}", e))?;

    // 记录 Token 消耗
    if let Some(usage) = json_resp.get("usage") {
        let prompt = usage.get("prompt_tokens").and_then(|v| v.as_i64()).unwrap_or(0);
        let completion = usage.get("completion_tokens").and_then(|v| v.as_i64()).unwrap_or(0);
        let total = usage.get("total_tokens").and_then(|v| v.as_i64()).unwrap_or(0);
        info!(
            prompt_tokens = prompt,
            completion_tokens = completion,
            total_tokens = total,
            "Token 消耗"
        );
    }

    // 记录 LLM 回复内容
    if let Some(choices) = json_resp.get("choices") {
        if let Some(first) = choices.as_array().and_then(|c| c.first()) {
            if let Some(message) = first.get("message") {
                let content = message.get("content").and_then(|v| v.as_str()).unwrap_or("(无文本回复)");
                info!("上游模型回复内容:\n{}", content);
            }
        }
    }

    if !status.is_success() {
        error!("上游 API 报错: {}", json_resp);
    }

    Ok(json_resp)
}
