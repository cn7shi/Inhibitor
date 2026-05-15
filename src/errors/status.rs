// errors/status.rs — 状态相关的错误类型
use std::fmt;
use crate::entity::status::Status;

/// 状态校验失败的错误
#[derive(Debug, PartialEq)]
pub enum StatusError {
    /// 状态不符合预期（期望状态, 实际状态）
    Unexpected {
        expected: Status,
        actual: Status,
    },
}

impl fmt::Display for StatusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StatusError::Unexpected { expected, actual } => {
                write!(f, "状态异常：期望 {}，实际为 {}", expected, actual)
            }
        }
    }
}
