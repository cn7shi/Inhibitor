// errors/status.rs — 状态相关的错误类型
use thiserror::Error;
use crate::entity::status::Status;

#[derive(Debug, Error, PartialEq)]
pub enum StatusError {
    #[error("状态异常：期望 {expected}，实际为 {actual}")]
    Unexpected { expected: Status, actual: Status },
}
