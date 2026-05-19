// validator/exit.rs — ExitGate 所需的全部校验，一次性收集所有错误

use crate::entity::permit::Permit;
use crate::errors::ExitGateError;
use super::{status::ensure_ready, permit::validate_permit_id, json};

/// 一次性运行所有出门校验，返回**所有**失败的错误
pub fn run(permit: &Permit) -> Result<(), Vec<ExitGateError>> {
    let mut errors: Vec<ExitGateError> = Vec::new();

    if let Err(e) = ensure_ready(permit.permit_status) {
        errors.push(e.into());
    }
    if let Err(e) = validate_permit_id(permit.permit_id) {
        errors.push(e.into());
    }
    // 出站只查空不空，不查格式（发给 LLM 的数据不需要是合法 JSON）
    if let Err(e) = json::check_not_empty(&permit.payload) {
        errors.push(e.into());
    }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

