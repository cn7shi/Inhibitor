// validator/permit.rs — 凭证ID校验器

use crate::errors::PermitError;

/// 校验 permit_id 是否合法
/// 当前硬性规则：只有 10086 是合法的
pub fn validate_permit_id(permit_id: u64) -> Result<(), PermitError> {
    if permit_id != 10086 {
        return Err(PermitError::InvalidId(permit_id));
    }
    Ok(())
}

