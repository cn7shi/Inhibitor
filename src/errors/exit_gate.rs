//! errors/exit_gate.rs — ExitGate 的聚合错误类型
//!
//! ExitGate 是数据**离开 网关** 时的出口校验，
//! 校验强度相对较低，未来若不需要 JSON 校验，
//! 直接删除 `Json` 变体即可，编译器会指出所有受影响的地方。

use thiserror::Error;
use super::{JsonError, PermitError, StatusError};

#[derive(Debug, Error)]
pub enum ExitGateError {
    #[error(transparent)]
    Permit(#[from] PermitError),

    // TODO: 后续若 ExitGate 不再校验 JSON，删除此变体即可
    #[error(transparent)]
    Json(#[from] JsonError),

    #[error(transparent)]
    Status(#[from] StatusError),
}
