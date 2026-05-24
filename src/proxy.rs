// proxy.rs — 薄 HTTP 代理适配层
// 职责：收 HTTP 请求 → 交给 pipeline 走完整生命周期 → 返回响应

use axum::{
    extract::Json,
    response::IntoResponse,
    http::StatusCode,
};
use tracing::{info, error};

pub async fn proxy_handler(
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    info!("========== 收到外部请求 (透明网关) ==========");
    info!(
        "转发的内容:\n{}",
        serde_json::to_string_pretty(&payload).unwrap_or_else(|_| "(序列化失败)".to_string())
    );

    // 委托 pipeline 执行完整生命周期：enroll → exit → execute → entry → finish
    match crate::pipeline::run_once(payload).await {
        Ok(json_resp) => {
            info!("响应获取完毕，正在原样返回给请求方...");
            (StatusCode::OK, Json(json_resp)).into_response()
        }
        Err(msg) => {
            error!("流水线执行失败: {}", msg);
            (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response()
        }
    }
}
