use crate::component::register::Registry;
use crate::gate::entry_gate::EntryGate;
use crate::gate::exit_gate::ExitGate;
use crate::strategies::circuit_breaker::CircuitBreaker;
use crate::component::groq_test::GroqTest;
use crate::config::Config;
use tracing::{info, error};

pub async fn run() {
    let config = Config::load().expect("配置加载失败");
    info!("=== 极简网关测试 ===");
    
    // 任务开始
    loop {
        let mut my_permit = Registry::enroll_task();
    
        // 打印Permit
        info!(permit_id = my_permit.permit_id, status = %my_permit.permit_status, "拿到凭证");

        /* 
            2.出门安检 (Exit)：准备离开agemt，去调用外部工具 （拿着刚初始化的 permit，检查状态是否干净）
        */

        // 用熔断器包裹：失败自动重试3次（日志由 Gate 和 CircuitBreaker 内部负责）
        if CircuitBreaker::retry(
            config.max_retries,
            || ExitGate::check_out(&mut my_permit),
            |_, _| {},
        ).is_err() {
            my_permit.block_with_log("exit");
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
        // （查验带回来的 permit_id 是否合法）
        // my_permit.permit_id = 10087;

        if CircuitBreaker::retry(
            config.max_retries,
            || EntryGate::check_in(&my_permit),
            |_, _| {},
        ).is_err() {
            my_permit.block_with_log("entry");
            return;
        }

        match my_permit.permit_status.finish() {
            Ok(done) => my_permit.permit_status = done,
            Err(msg) => error!(error = msg.as_str(), "状态转换异常"),
        }
        info!(status = %my_permit.permit_status, "工具执行完毕，数据安全回到agent，进入下一轮思考。");

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
