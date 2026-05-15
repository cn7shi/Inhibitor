//! errors/entry_gate.rs — EntryGate 的聚合错误类型
//!
//! EntryGate 是数据**回到 网关** 时的入口校验，
//! 校验强度较高，未来可能扩展 Schema、语义等更严格的检查。

use thiserror::Error;
use super::{JsonError, PermitError, StatusError};

#[derive(Debug, Error)]
pub enum EntryGateError {
    #[error(transparent)]
    Permit(#[from] PermitError),

    #[error(transparent)]
    Json(#[from] JsonError),

    #[error(transparent)]
    Status(#[from] StatusError),
}
