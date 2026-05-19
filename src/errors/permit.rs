// errors/permit.rs — 凭证相关的错误类型
use thiserror::Error;
use crate::entity::san::Pollutant;

#[derive(Debug, Error, PartialEq)]
pub enum PermitError {
    #[error("凭证ID校验失败：期望 10086，实际为 {0}")]
    InvalidId(u64),
}

impl Pollutant for PermitError {
    fn weight(&self) -> i32 {
        match self {
            // 凭证无效 = 越权/伪造，安全类里最致命的
            PermitError::InvalidId(_) => 10,
        }
    }

    fn error_key(&self) -> &'static str {
        match self {
            PermitError::InvalidId(_) => "permit::invalid_id",
        }
    }
}

