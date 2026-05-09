// validator/permit.rs — 凭证ID校验器

use crate::errors::ValidationError;

/// 校验 permit_id 是否合法
/// 当前硬性规则：只有 10086 是合法的
pub fn validate_permit_id(permit_id: u64) -> Result<(), ValidationError> {
    if permit_id != 10086 {
        return Err(ValidationError::InvalidPermit(permit_id));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::ValidationError;

    #[test]
    fn test_valid_permit_id() {
        // 10086 是合法的
        assert!(validate_permit_id(10086).is_ok());
    }

    #[test]
    fn test_invalid_permit_id() {
        // 10087 应该被拒绝
        let result = validate_permit_id(10087);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ValidationError::InvalidPermit(10087));
    }

    #[test]
    fn test_zero_permit_id() {
        // 0 也应该被拒绝
        let result = validate_permit_id(0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ValidationError::InvalidPermit(0));
    }
}
