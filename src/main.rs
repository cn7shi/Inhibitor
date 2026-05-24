// src/main.rs

mod entity;
mod component;
mod config;
mod gate;
mod strategies;
mod validator;
mod errors;
mod telemetry;
mod proxy;
mod pipeline;

#[tokio::main]
async fn main() {
    // 初始化日志订阅器和广播管道
    let tx = telemetry::setup_tracing();
    
    // 启动日志监控后端服务（含透明网关路由）
    tokio::spawn(telemetry::start_server(tx));
    
    // 给服务器一点时间启动
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    tracing::info!("网关已就绪，等待外部请求...");
    tracing::info!("路由：POST http://127.0.0.1:3000/proxy/v1/chat/completions");

    // 等待 Ctrl+C 信号优雅退出
    tokio::signal::ctrl_c().await.expect("无法监听 Ctrl+C");
    tracing::info!("收到退出信号，网关关闭");
}

// 执行流水线（由 gateway 透明触发，每个请求独立执行一轮）
/*
    2026-5-24 改造
    HTTP 请求到达 → enroll → exit → execute → entry → finish → 返回响应
    
    每次请求拥有专属的 permit_id，在 pipeline::run_once() 中走完整生命周期。
*/

