// src/main.rs

mod entity;
mod component;
mod config;
mod gate;
mod strategies;
mod validator;
mod errors;
mod telemetry;
mod gateway;
mod pipeline;

#[tokio::main]
async fn main() {
    // 初始化日志订阅器和广播管道
    let tx = telemetry::setup_tracing();
    
    // 启动日志监控后端服务
    tokio::spawn(telemetry::start_server(tx));
    
    // 给服务器一点时间启动
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // 启动核心流水线
    pipeline::run().await;
}

//执行流水线
/*
    2026-5-6  
    enroll->exit->execute->entry->execute...
    
    任务先在内部注册，随后，经过出参校验后，开始调用模型API，
    API返回调用结果，再经过入参校验，取得调用结果。
*/
