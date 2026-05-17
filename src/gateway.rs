use axum::{
    extract::Json,
    response::IntoResponse,
    http::StatusCode,
};
use tracing::{info, error};

pub async fn proxy_handler(
    Json(mut payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    info!("========== 收到外部请求 (透明网关) ==========");
    info!("转发的内容:\n{}", serde_json::to_string_pretty(&payload).unwrap_or_default());

    let cfg = match crate::config::Config::load() {
        Ok(c) => c,
        Err(e) => {
            error!("加载配置失败: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Config Error").into_response();
        }
    };

    // 【新增逻辑】：拦截并覆盖模型名称
    // 强制把别人传过来的模型名称，替换成我们 config.toml 里配置的模型（例如 "openai/gpt-oss-120b"）
    if let Some(obj) = payload.as_object_mut() {
        obj.insert("model".to_string(), serde_json::json!(cfg.model));
    }
    info!("已强制将请求模型重定向为系统默认模型: {}", cfg.model);

    let client = reqwest::Client::new();
    
    info!("正在将请求原样转发给上游 LLM (Groq)...");
    // 这里暂时硬编码转发到 Groq 的标准 OpenAI 接口，后续可以做成可配置的
    let resp = match client
        .post("https://api.groq.com/openai/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", cfg.groq_api_key))
        .json(&payload)
        .send()
        .await
    {
        Ok(r) => r,
        Err(e) => {
            error!("代理转发失败: {}", e);
            return (StatusCode::BAD_GATEWAY, "Proxy Error").into_response();
        }
    };

    let status = resp.status();
    let json_resp: serde_json::Value = match resp.json().await {
        Ok(j) => j,
        Err(e) => {
            error!("解析上游响应失败: {}", e);
            return (StatusCode::BAD_GATEWAY, "Bad Gateway Response").into_response();
        }
    };

    // 尝试提取并记录 Token 消耗
    if let Some(usage) = json_resp.get("usage") {
        let prompt = usage.get("prompt_tokens").and_then(|v| v.as_i64()).unwrap_or(0);
        let completion = usage.get("completion_tokens").and_then(|v| v.as_i64()).unwrap_or(0);
        let total = usage.get("total_tokens").and_then(|v| v.as_i64()).unwrap_or(0);
        
        info!(
            prompt_tokens = prompt,
            completion_tokens = completion,
            total_tokens = total,
            "========== Token 消耗 (透明网关) =========="
        );
    } else {
        info!("========== 上游响应 (无 Token 统计) ==========");
    }
    
    // 如果是错误响应，顺便打印出来
    if !status.is_success() {
        error!("上游 API 报错: {}", json_resp);
    }

    // 尝试提取大模型的具体回复内容并记录到日志
    if let Some(choices) = json_resp.get("choices") {
        if let Some(first_choice) = choices.as_array().and_then(|c| c.first()) {
            if let Some(message) = first_choice.get("message") {
                let content = message.get("content").and_then(|v| v.as_str()).unwrap_or("(无文本回复)");
                info!("========== 上游模型回复内容 ==========\n{}", content);
            }
        }
    }

    info!("响应获取完毕，正在原样返回给请求方...");

    // 原样打包返回给外部的 Agent
    (status, Json(json_resp)).into_response()
}
