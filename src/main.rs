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
use errors::ValidationError;



#[tokio::main]
async fn main() {
    println!("=== 极简网关测试 ===");
    
    //任务开始  测试加mut
    let  my_permit = Registry::enroll_task();
    
    // 打印Permit
    println!("[内部登记] 拿到的凭证: {:#?}", my_permit);
    // my_permit.permit_id = 10087;

    /* 
        2.出门安检 (Exit)：准备离开agemt，去调用外部工具 （拿着刚初始化的 permit，检查状态是否干净）
    */
   
    // 用熔断器包裹：失败自动重试3次
    if let Err(e) = CircuitBreaker::retry(DEFAULT_MAX_RETRIES, || {
        ExitGate::check_out(&my_permit)
    }) {
        match &e {
            ValidationError::InvalidPermit(id) => println!("🔴 [出门守卫] 凭证异常 (ID: {})", id),
            ValidationError::InvalidJson(err) => println!("🔴 [出门守卫] JSON 异常: {}", err),
            ValidationError::InvalidStatus(s) => println!("🔴 [出门守卫] 状态异常 (Status: {})", s),
        }
        return; // 重试3次都失败，才真正放弃
    }

    // 3. 核心执行 (Execute)：调用 Groq API
    println!("[外部执行] 离开agent，正在调用 Groq API...");
    match GroqTest::call("你好，请用一句话介绍你自己。").await {
        Ok(reply) => {
            println!("[外部执行] ✅ API 返回结果:");
            println!("{}", reply);
        }
        Err(e) => {
            println!("[外部执行] {}", e);
            return;
        }
    }
    

    // 4. 进门安检 (Entry)：工具带着结果回来了，准备进入agent
    // （查验带回来的 permit_id 是否合法）
    // my_permit.permit_id = 10087;
    
    if let Err(e) = CircuitBreaker::retry(DEFAULT_MAX_RETRIES, || {
        EntryGate::check_in(&my_permit)
    }) {
        match &e {
            ValidationError::InvalidPermit(id) => println!("🔴 [进门守卫] 凭证异常 (ID: {})", id),
            ValidationError::InvalidJson(err) => println!("🔴 [进门守卫] JSON 异常: {}", err),
            ValidationError::InvalidStatus(s) => println!("🔴 [进门守卫] 状态异常 (Status: {})", s),
        }
        /*
           待补充决策：未来按错误类型触发不同策略
        */
        return; // 查验不合格，拦截在门外！
    }

    println!("[主流程] 恭喜！工具执行完毕，数据安全回到agent，进入下一轮思考。");

    // ===== 工具调用测试 =====
    println!("\n=== 工具调用测试 ===");
    match GroqTest::call_with_tools("What's the weather in San Francisco?").await {
        Ok(reply) => {
            println!("[工具调用] ✅ 最终回复:");
            println!("{}", reply);
        }
        Err(e) => {
            println!("[工具调用] ❌ {}", e);
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
