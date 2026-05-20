// retry.rs
// 通用重试器：失败就重试，每次失败调用 on_error 回调
// on_error 返回 bool：true = 继续重试，false = 立即终止（如 SAN 归零）
// 供非 gate 场景使用（API 调用、数据库重连等）

use tracing::{warn, error, info};

pub struct Retry;

impl Retry {
    /// 带回调的重试：每次失败都调用 on_error，让外部感知并调整策略
    /// on_error 返回 true 表示"继续重试"，返回 false 表示"立即终止"
    /// E 不要求 Display：错误的打印职责完全由 on_error 回调承担
    pub fn execute<T, E, F, H>(
        max_retries: u32,
        mut operation: F,
        mut on_error: H,
    ) -> Result<T, E>
    where
        F: FnMut() -> Result<T, E>,
        H: FnMut(&E, u32) -> bool,   // (错误引用, 第几次尝试) → true=继续, false=立即停
    {
        let mut last_err;

        // 首次执行 (attempt = 0)
        match operation() {
            Ok(val) => return Ok(val),
            Err(e) => {
                warn!(component = "retry", attempt = 0, "执行失败");
                if !on_error(&e, 0) {
                    error!(component = "retry", "回调要求立即终止，跳过剩余重试");
                    return Err(e);
                }
                last_err = e;
            }
        }

        // 重试 (attempt = 1..=max_retries)
        for attempt in 1..=max_retries {
            info!(component = "retry", attempt = attempt, max = max_retries, "正在重试...");
            match operation() {
                Ok(val) => {
                    info!(component = "retry", attempt = attempt, "重试成功");
                    return Ok(val);
                }
                Err(e) => {
                    warn!(component = "retry", attempt = attempt, "重试失败");
                    if !on_error(&e, attempt) {
                        error!(component = "retry", attempt = attempt, "回调要求立即终止，跳过剩余重试");
                        return Err(e);
                    }
                    last_err = e;
                }
            }
        }

        error!(component = "retry", max_retries = max_retries, "重试耗尽");
        Err(last_err)
    }
}
