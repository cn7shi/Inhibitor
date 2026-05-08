// circuitbreaker.rs
// 最简熔断器：失败就重试，最多 max_retries 次

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
                println!("🟡 [熔断器] 首次执行失败: {}", e);
                last_err = e;
            }
        }

        // 重试
        for attempt in 1..=max_retries {
            println!("🔄 [熔断器] 第 {} 次重试...", attempt);
            match operation() {
                Ok(val) => {
                    println!("🟢 [熔断器] 第 {} 次重试成功", attempt);
                    return Ok(val);
                }
                Err(e) => {
                    println!("🟡 [熔断器] 第 {} 次重试失败: {}", attempt, e);
                    last_err = e;
                }
            }
        }

        println!("🔴 [熔断器] 重试耗尽，触发硬熔断");
        Err(last_err)
    }
}
