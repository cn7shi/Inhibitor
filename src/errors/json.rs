// errors/json.rs — JSON 相关的错误类型
use thiserror::Error;
use crate::entity::san::Pollutant;

#[derive(Debug, Error)]
pub enum JsonError {
    #[error("JSON 校验失败：内容为空")]
    Empty,

    #[error("JSON 校验失败：{0}")]
    ParseFailed(#[from] serde_json::Error),
}

impl Pollutant for JsonError {
    fn weight(&self) -> i32 {
        match self {
            // 空内容，可能 LLM 就是没输出，属于正常波动
            JsonError::Empty => 2,
            // 格式解析失败，需要关注但通常可重试
            JsonError::ParseFailed(_) => 5,
        }
    }

    fn error_key(&self) -> &'static str {
        match self {
            JsonError::Empty => "json::empty",
            JsonError::ParseFailed(_) => "json::parse_failed",
        }
    }
}

