// errors/permit.rs — 凭证相关的错误类型
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum PermitError {
    #[error("凭证ID校验失败：期望 10086，实际为 {0}")]
    InvalidId(u64),
}
