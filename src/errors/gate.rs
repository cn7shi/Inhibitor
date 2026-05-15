// errors/gate.rs — Gate 层的聚合错误类型
// 把各个子校验的错误收拢在一起，供 main.rs 等调用层统一处理
use std::fmt;
use super::{PermitError, JsonError, StatusError};

/// Gate 校验失败时可能出现的错误（聚合类型）
#[derive(Debug)]
pub enum GateError {
    Permit(PermitError),
    Json(JsonError),
    Status(StatusError),
}

impl fmt::Display for GateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GateError::Permit(e) => write!(f, "{}", e),
            GateError::Json(e)   => write!(f, "{}", e),
            GateError::Status(e) => write!(f, "{}", e),
        }
    }
}

// 自动转换：? 运算符可以从子错误直接升级到 GateError
impl From<PermitError> for GateError {
    fn from(e: PermitError) -> Self { GateError::Permit(e) }
}

impl From<JsonError> for GateError {
    fn from(e: JsonError) -> Self { GateError::Json(e) }
}

impl From<StatusError> for GateError {
    fn from(e: StatusError) -> Self { GateError::Status(e) }
}
