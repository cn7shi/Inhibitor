//! errors/exit_gate.rs — ExitGate 的聚合错误类型
//!
//! ExitGate 是数据**离开 网关** 时的出口校验，
//! 校验强度相对较低，未来若不需要 JSON 校验，
//! 直接删除 `Json` 变体即可，编译器会指出所有受影响的地方。

use thiserror::Error;
use super::{JsonError, PermitError, StatusError};
use crate::entity::san::Pollutant;

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

/// 出站场景乘数
const MULTIPLIER_JSON: i32 = 1;
const MULTIPLIER_PERMIT: i32 = 10;
const MULTIPLIER_STATUS: i32 = 2;

impl Pollutant for ExitGateError {
    fn weight(&self) -> i32 {
        match self {
            // 出站不太在乎格式（发给 LLM，它能理解）
            ExitGateError::Json(e) => e.weight() * MULTIPLIER_JSON,
            // 安全一样零容忍
            ExitGateError::Permit(e) => e.weight() * MULTIPLIER_PERMIT,
            // 出站逻辑问题更严重（逻辑都错了还往外发）
            ExitGateError::Status(e) => e.weight() * MULTIPLIER_STATUS,
        }
    }

    fn error_key(&self) -> &'static str {
        match self {
            ExitGateError::Json(e) => e.error_key(),
            ExitGateError::Permit(e) => e.error_key(),
            ExitGateError::Status(e) => e.error_key(),
        }
    }
}

