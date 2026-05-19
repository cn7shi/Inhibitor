// validator/entry.rs — EntryGate 所需的全部校验，一次性收集所有错误

use crate::entity::permit::Permit;
use crate::errors::EntryGateError;
use super::{status::ensure_running, permit::validate_permit_id, json};

/// 一次性运行所有入门校验，返回**所有**失败的错误
/// 即使前面的校验失败，也继续检查后面的，避免一次只发现一个问题
pub fn run(permit: &Permit) -> Result<(), Vec<EntryGateError>> {
    let mut errors: Vec<EntryGateError> = Vec::new();

    if let Err(e) = ensure_running(permit.permit_status) {
        errors.push(e.into());
    }
    if let Err(e) = validate_permit_id(permit.permit_id) {
        errors.push(e.into());
    }
    // 入站两个都查：空不空 + 格式对不对（LLM 返回必须能解析）
    if let Err(e) = json::check_not_empty(&permit.payload) {
        errors.push(e.into());
    }
    if let Err(e) = json::check_format(&permit.payload) {
        errors.push(e.into());
    }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

