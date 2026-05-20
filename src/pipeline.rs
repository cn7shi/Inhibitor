use crate::component::register::Registry;
use crate::gate::entry_gate::EntryGate;
use crate::gate::exit_gate::ExitGate;
use crate::component::groq_test::GroqTest;
use crate::component::notify::Notify;
use crate::entity::permit::Permit;
use crate::entity::san::{Pollutant, SanSchema};
use crate::config::Config;
use tracing::{info, warn, error};

/// 通用 Gate 校验：重试 + SAN 扣减 + 失败处理
/// 返回 true = 校验通过，false = 已触发 Blocked
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
                    san.apply_penalty(penalty, e.error_key());
                }
                if san.is_corrupted() {
                    error!(component = gate_name, "SAN 归零，立即终止重试");
                    break;
                }
            }
        }
    }

    // 走到这里 = 重试耗尽 或 SAN 归零
    permit.san = san; // 放回
    permit.block_with_log(gate_name);
    if permit.san.is_corrupted() {
        warn!(san = permit.san.current_san, "SAN 值归零，环境已被严重污染");
    }
    Notify::send(
        "任务挂起 (Blocked)",
        &format!("{} Gate 校验失败，ID: {}，SAN: {}/{}",
            gate_name, permit.permit_id, permit.san.current_san, permit.san.max_san),
    ).await;
    false
}

pub async fn run() {
    info!("=== 极简网关测试 ===");
    
    //目前得架构还是一个粗浅模拟，gateway没用走permit申请
    // 任务开始
    loop {
        // 在循环内部加载配置，以支持热更新（例如 max_retries 的改变）
        let config = Config::load().expect("配置加载失败");

        let mut my_permit = Registry::enroll_task();
    
        // 打印Permit
        info!(permit_id = my_permit.permit_id, status = %my_permit.permit_status, san = my_permit.san.current_san, "拿到凭证");

        // 2. 出门安检 (Exit)：准备离开agent，去调用外部工具
        if !run_gate(&mut my_permit, &config, "Exit", |p| ExitGate::check_out(p)).await {
            return;
        }

        // 3. 核心执行 (Execute)：调用 Groq API
        // info!("离开agent，正在调用 Groq API...");
        // match GroqTest::call("你好，请用一句话介绍你自己。").await {
        //     Ok(reply) => {
        //         info!("API 返回结果:");
        //         println!("{}", reply);
        //     }
        //     Err(e) => {
        //         error!(error = %e, "外部执行失败");
        //         return;
        //     }
        // }

        // 4. 进门安检 (Entry)：工具带着结果回来了，准备进入agent
        // my_permit.permit_id = 10087;
        if !run_gate(&mut my_permit, &config, "Entry", |p| EntryGate::check_in(p)).await {
            return;
        }

        my_permit.finish_with_log();
        Notify::send("任务完成 (Done)", &format!(
            "任务完成，ID: {}，最终SAN: {}/{}",
            my_permit.permit_id, my_permit.san.current_san, my_permit.san.max_san
        )).await;

        // ===== 工具调用测试 =====
        info!("=== 工具调用测试 ===");
        match GroqTest::call_with_tools("What's the weather in San Francisco?").await {
            Ok(reply) => {
                info!("工具调用最终回复:");
                println!("{}", reply);
            }
            Err(e) => {
                error!(error = %e, "工具调用失败");
            }
        }

        info!("所有任务执行完毕，等待180秒后进行下一轮...");
        tokio::time::sleep(tokio::time::Duration::from_secs(180)).await;
    }
}
