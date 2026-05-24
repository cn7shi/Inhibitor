// validator/permit.rs — 凭证ID校验器

use crate::errors::PermitError;

/// 校验 permit_id 是否合法
/// 规则：ID 必须大于 0（即由 Registry 动态签发）
pub fn validate_permit_id(permit_id: u64) -> Result<(), PermitError> {
    if permit_id == 0 {
        return Err(PermitError::InvalidId(permit_id));
    }
    Ok(())
}

