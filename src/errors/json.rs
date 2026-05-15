// errors/json.rs — JSON 相关的错误类型
use thiserror::Error;

#[derive(Debug, Error)]
pub enum JsonError {
    #[error("JSON 校验失败：内容为空")]
    Empty,

    #[error("JSON 校验失败：{0}")]
    ParseFailed(#[from] serde_json::Error),
}
