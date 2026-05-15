// errors/permit.rs — 凭证相关的错误类型
use std::fmt;

/// 凭证校验失败的错误
#[derive(Debug, PartialEq)]
pub enum PermitError {
    /// 凭证ID不合法（携带实际收到的ID）
    InvalidId(u64),
}

impl fmt::Display for PermitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PermitError::InvalidId(id) => {
                write!(f, "凭证ID校验失败：期望 10086，实际为 {}", id)
            }
        }
    }
}
