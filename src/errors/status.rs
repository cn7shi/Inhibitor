// errors/status.rs — 状态相关的错误类型
use thiserror::Error;
use crate::entity::status::Status;
use crate::entity::san::Pollutant;

#[derive(Debug, Error, PartialEq)]
pub enum StatusError {
    #[error("状态异常：期望 {expected}，实际为 {actual}")]
    Unexpected { expected: Status, actual: Status },
}

/// 状态错误固有权重
const WEIGHT_UNEXPECTED: i32 = 5;

impl Pollutant for StatusError {
    fn weight(&self) -> i32 {
        match self {
            // 状态不一致，逻辑流程出了问题，中等严重
            StatusError::Unexpected { .. } => WEIGHT_UNEXPECTED,
        }
    }

    fn error_key(&self) -> &'static str {
        match self {
            StatusError::Unexpected { .. } => "status::unexpected",
        }
    }
}

