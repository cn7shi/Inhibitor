// errors/gate.rs — Gate 层的聚合错误类型
use thiserror::Error;
use super::{JsonError, PermitError, StatusError};

#[derive(Debug, Error)]
pub enum GateError {
    #[error(transparent)]
    Permit(#[from] PermitError),

    #[error(transparent)]
    Json(#[from] JsonError),

    #[error(transparent)]
    Status(#[from] StatusError),
}
