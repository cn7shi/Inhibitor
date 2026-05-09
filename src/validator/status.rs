// validator/status.rs — 状态码校验器

use crate::errors::ValidationError;

/// 校验 permit_status 是否合法
/// 当前规则：只有 0 (Ready) 是合法的
pub fn validate_status(status: u8) -> Result<(), ValidationError> {
    if status != 0 {
        return Err(ValidationError::InvalidStatus(status));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::ValidationError;

    #[test]
    fn test_valid_status() {
        // 0 = Ready，合法
        assert!(validate_status(0).is_ok());
    }

    #[test]
    fn test_invalid_status() {
        // 非0状态应该被拒绝
        let result = validate_status(1);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ValidationError::InvalidStatus(1));
    }

    #[test]
    fn test_status_255() {
        // 极端值也应该被拒绝
        let result = validate_status(255);
        assert_eq!(result.unwrap_err(), ValidationError::InvalidStatus(255));
    }
}
