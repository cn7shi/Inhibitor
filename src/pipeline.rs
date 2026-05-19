use crate::component::register::Registry;
use crate::gate::entry_gate::EntryGate;
use crate::gate::exit_gate::ExitGate;
use crate::strategies::circuit_breaker::CircuitBreaker;
use crate::component::groq_test::GroqTest;
use crate::component::notify::Notify;
use crate::entity::san::{Pollutant, SanSchema};
use crate::config::Config;
use tracing::{info, warn, error};

pub async fn run() {
    info!("=== 极简网关测试 ===");
    
    // 任务开始
    loop {
        // 在循环内部加载配置，以支持热更新（例如 max_retries 的改变）
        let config = Config::load().expect("配置加载失败");

        let mut my_permit = Registry::enroll_task();
    
        // 打印Permit
        info!(permit_id = my_permit.permit_id, status = %my_permit.permit_status, san = my_permit.san.current_san, "拿到凭证");

        /* 
            2.出门安检 (Exit)：准备离开agemt，去调用外部工具
        */

        // 临时取出 SAN，避免 &mut my_permit 和 &mut my_permit.san 的借用冲突
        // Gate 校验不访问 san 字段，所以临时替换为空值是安全的
        let mut san = std::mem::replace(&mut my_permit.san, SanSchema::new(0));
        let exit_result = CircuitBreaker::retry(
            config.max_retries,
            || ExitGate::check_out(&mut my_permit),
            |errors, _| {
                for e in errors {
                    let penalty = config.san_overrides.get(e.error_key()).copied().unwrap_or_else(|| e.weight());
                    san.apply_penalty(penalty, e.error_key());
                }
            },
        );
        my_permit.san = san; // 放回

        if exit_result.is_err() {
            my_permit.block_with_log("exit");
            if my_permit.san.is_corrupted() {
                warn!(san = my_permit.san.current_san, "SAN 值归零，环境已被严重污染");
            }
            Notify::send("任务挂起 (Blocked)", &format!("Exit Gate 校验失败，ID: {}，SAN: {}/{}", my_permit.permit_id, my_permit.san.current_san, my_permit.san.max_san)).await;
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

        let mut san = std::mem::replace(&mut my_permit.san, SanSchema::new(0));
        let entry_result = CircuitBreaker::retry(
            config.max_retries,
            || EntryGate::check_in(&my_permit),
            |errors, _| {
                for e in errors {
                    let penalty = config.san_overrides.get(e.error_key()).copied().unwrap_or_else(|| e.weight());
                    san.apply_penalty(penalty, e.error_key());
                }
            },
        );
        my_permit.san = san; // 放回

        if entry_result.is_err() {
            my_permit.block_with_log("entry");
            if my_permit.san.is_corrupted() {
                warn!(san = my_permit.san.current_san, "SAN 值归零，环境已被严重污染");
            }
            Notify::send("任务挂起 (Blocked)", &format!("Entry Gate 校验失败，ID: {}，SAN: {}/{}", my_permit.permit_id, my_permit.san.current_san, my_permit.san.max_san)).await;
            return;
        }

        match my_permit.permit_status.finish() {
            Ok(done) => {
                my_permit.permit_status = done;
                Notify::send("任务完成 (Done)", &format!("任务完成，ID: {}，最终SAN: {}/{}", my_permit.permit_id, my_permit.san.current_san, my_permit.san.max_san)).await;
            }
            Err(msg) => error!(error = msg.as_str(), "状态转换异常"),
        }
        info!(status = %my_permit.permit_status, san = my_permit.san.current_san, "工具执行完毕，数据安全回到agent，进入下一轮思考。");

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
