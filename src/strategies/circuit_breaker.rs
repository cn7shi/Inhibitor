// circuitbreaker.rs
// 最简熔断器：失败就重试，每次失败调用 on_error 回调

use tracing::{warn, error, info};

pub struct CircuitBreaker;

impl CircuitBreaker {
    /// 带回调的重试：每次失败都调用 on_error，让外部感知并调整策略
    /// E 不要求 Display：错误的打印职责完全由 on_error 回调承担
    pub fn retry<T, E, F, H>(
        max_retries: u32,
        mut operation: F,
        mut on_error: H,
    ) -> Result<T, E>
    where
        F: FnMut() -> Result<T, E>,
        H: FnMut(&E, u32),   // (错误引用, 第几次尝试)
    {
        let mut last_err;

        // 首次执行 (attempt = 0)
        match operation() {
            Ok(val) => return Ok(val),
            Err(e) => {
                warn!(component = "circuit_breaker", attempt = 0, "执行失败");
                on_error(&e, 0);
                last_err = e;
            }
        }

        // 重试 (attempt = 1..=max_retries)
        for attempt in 1..=max_retries {
            info!(component = "circuit_breaker", attempt = attempt, max = max_retries, "正在重试...");
            match operation() {
                Ok(val) => {
                    info!(component = "circuit_breaker", attempt = attempt, "重试成功");
                    return Ok(val);
                }
                Err(e) => {
                    warn!(component = "circuit_breaker", attempt = attempt, "重试失败");
                    on_error(&e, attempt);
                    last_err = e;
                }
            }
        }

        error!(component = "circuit_breaker", max_retries = max_retries, "重试耗尽，触发硬熔断");
        Err(last_err)
    }
}
