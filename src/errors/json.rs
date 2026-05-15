// errors/json.rs — JSON 相关的错误类型
use std::fmt;

/// JSON 校验失败的错误
#[derive(Debug, PartialEq)]
pub enum JsonError {
    /// JSON格式不合法（携带原始解析错误信息）
    Invalid(String),
}

impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JsonError::Invalid(err) => {
                write!(f, "JSON 校验失败：{}", err)
            }
        }
    }
}
