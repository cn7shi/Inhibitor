// src/main.rs

mod entity;
mod component;
mod config;
mod constant;
mod gate;
mod strategies;
mod validator;
mod errors;


use component::register::Registry;
use gate::entrygate::EntryGate;
use gate::exitgate::ExitGate;
use strategies::circuit_breaker::CircuitBreaker;
use component::groq_test::GroqTest;
use constant::DEFAULT_MAX_RETRIES;
use errors::GateError;
use tracing::{info, warn, error};



#[tokio::main]
async fn main() {
    // 初始化日志订阅器
    tracing_subscriber::fmt()
        .with_target(false)
        .init();

    info!("=== 极简网关测试 ===");
    
    //任务开始  测试加mut
    let mut my_permit = Registry::enroll_task();
    
    // 打印Permit
    info!(permit_id = my_permit.permit_id, status = %my_permit.permit_status, "拿到凭证");

    /* 
        2.出门安检 (Exit)：准备离开agemt，去调用外部工具 （拿着刚初始化的 permit，检查状态是否干净）
    */
   
    // 用熔断器包裹：失败自动重试3次
    if let Err(_e) = CircuitBreaker::retry(
        DEFAULT_MAX_RETRIES,
        || ExitGate::check_out(&mut my_permit),
        |err, attempt| {
            // 每次失败的回调 —— 未来 planner 在这里根据错误类型调整重试参数
            match err {
                GateError::Permit(e) => warn!(gate = "exit", attempt = attempt, error = %e, "凭证异常"),
                GateError::Json(e)   => warn!(gate = "exit", attempt = attempt, error = %e, "JSON 异常"),
                GateError::Status(e) => warn!(gate = "exit", attempt = attempt, error = %e, "状态异常"),
            }
        },
    ) {
        match my_permit.permit_status.block() {
            Ok(blocked) => my_permit.permit_status = blocked,
            Err(msg) => error!(error = msg.as_str(), "状态转换异常"),
        }
        error!(status = %my_permit.permit_status, "出门校验最终失败，任务已挂起");
        return;
    }

    // 3. 核心执行 (Execute)：调用 Groq API
    info!("离开agent，正在调用 Groq API...");
    match GroqTest::call("你好，请用一句话介绍你自己。").await {
        Ok(reply) => {
            info!("API 返回结果:");
            println!("{}", reply);
        }
        Err(e) => {
            error!(error = %e, "外部执行失败");
            return;
        }
    }
    

    // 4. 进门安检 (Entry)：工具带着结果回来了，准备进入agent
    // （查验带回来的 permit_id 是否合法）
    // my_permit.permit_id = 10087;
    
    if let Err(_e) = CircuitBreaker::retry(
        DEFAULT_MAX_RETRIES,
        || EntryGate::check_in(&my_permit),
        |err, attempt| {
            // 每次失败的回调 —— 未来 planner 在这里根据错误类型调整重试参数
            match err {
                GateError::Permit(e) => warn!(gate = "entry", attempt = attempt, error = %e, "凭证异常"),
                GateError::Json(e)   => warn!(gate = "entry", attempt = attempt, error = %e, "JSON 异常"),
                GateError::Status(e) => warn!(gate = "entry", attempt = attempt, error = %e, "状态异常"),
            }
        },
    ) {
        match my_permit.permit_status.block() {
            Ok(blocked) => my_permit.permit_status = blocked,
            Err(msg) => error!(error = msg.as_str(), "状态转换异常"),
        }
        error!(status = %my_permit.permit_status, "进门校验最终失败，任务已挂起");
        return;
    }

    info!("工具执行完毕，数据安全回到agent，进入下一轮思考。");

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



    
}
//执行流水线
/*
    2026-5-6  
    enroll->exit->execute->entry->execute...
    
    任务先在内部注册，随后，经过出参校验后，开始调用模型API，
    API返回调用结果，再经过入参校验，取得调用结果。
*/
