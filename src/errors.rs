// errors.rs — 统一错误类型
// 描述相关错误
use std::fmt;

/// 校验相关的错误类型
#[derive(Debug)]
pub enum ValidationError {
    /// 凭证ID不合法（携带实际收到的ID）
    InvalidPermit(u64),
    /// JSON格式不合法（携带原始解析错误）
    InvalidJson(String),
    /// 状态码异常（携带实际收到的状态码）
    InvalidStatus(u8),
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::InvalidPermit(id) => {
                write!(f, "凭证ID校验失败：期望 10086，实际为 {}", id)
            }
            ValidationError::InvalidJson(err) => {
                write!(f, "JSON 校验失败：{}", err)
            }
            ValidationError::InvalidStatus(status) => {
                write!(f, "状态码异常：期望 0 (Ready)，实际为 {}", status)
            }
        }
    }
}
