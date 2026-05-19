//! errors/entry_gate.rs — EntryGate 的聚合错误类型
//!
//! EntryGate 是数据**回到 网关** 时的入口校验，
//! 校验强度较高，未来可能扩展 Schema、语义等更严格的检查。

use thiserror::Error;
use super::{JsonError, PermitError, StatusError};
use crate::entity::san::Pollutant;

#[derive(Debug, Error)]
pub enum EntryGateError {
    #[error(transparent)]
    Permit(#[from] PermitError),

    #[error(transparent)]
    Json(#[from] JsonError),

    #[error(transparent)]
    Status(#[from] StatusError),
}

/// 入站场景乘数
const MULTIPLIER_JSON: i32 = 3;
const MULTIPLIER_PERMIT: i32 = 10;
const MULTIPLIER_STATUS: i32 = 1;

impl Pollutant for EntryGateError {
    fn weight(&self) -> i32 {
        match self {
            // 入站对格式极度敏感（LLM 返回必须能解析）
            EntryGateError::Json(e) => e.weight() * MULTIPLIER_JSON,
            // 安全零容忍
            EntryGateError::Permit(e) => e.weight() * MULTIPLIER_PERMIT,
            // 状态问题正常权重
            EntryGateError::Status(e) => e.weight() * MULTIPLIER_STATUS,
        }
    }

    fn error_key(&self) -> &'static str {
        match self {
            EntryGateError::Json(e) => e.error_key(),
            EntryGateError::Permit(e) => e.error_key(),
            EntryGateError::Status(e) => e.error_key(),
        }
    }
}

