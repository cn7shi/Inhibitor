// circuitbreaker.rs
// 最简熔断器：失败就重试，最多 max_retries 次

use tracing::{warn, error, info};

pub struct CircuitBreaker;

impl CircuitBreaker {
    /// 传入一个闭包，失败就重试，最多重试 max_retries 次
    pub fn retry<T, E, F>(max_retries: u32, mut operation: F) -> Result<T, E>
    where
        F: FnMut() -> Result<T, E>,
        E: std::fmt::Display,
    {
        let mut last_err;

        // 首次执行
        match operation() {
            Ok(val) => return Ok(val),
            Err(e) => {
                warn!(component = "circuit_breaker", error = %e, "首次执行失败");
                last_err = e;
            }
        }

        // 重试
        for attempt in 1..=max_retries {
            warn!(component = "circuit_breaker", attempt = attempt, max = max_retries, "正在重试...");
            match operation() {
                Ok(val) => {
                    info!(component = "circuit_breaker", attempt = attempt, "重试成功");
                    return Ok(val);
                }
                Err(e) => {
                    warn!(component = "circuit_breaker", attempt = attempt, error = %e, "重试失败");
                    last_err = e;
                }
            }
        }

        error!(component = "circuit_breaker", max_retries = max_retries, "重试耗尽，触发硬熔断");
        Err(last_err)
    }
}
