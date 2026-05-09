// validator/json.rs — JSON 格式校验器
//只管校验对错，不管校验出什么结果
use crate::errors::ValidationError;

/// 校验 JSON 字符串是否合法
/// 1. 不能为空
/// 2. 必须是合法的 JSON 格式
pub fn validate_json(payload: &str) -> Result<(), ValidationError> {
    if payload.trim().is_empty() {
        return Err(ValidationError::InvalidJson("内容为空".to_string()));
    }

    if let Err(e) = serde_json::from_str::<serde_json::Value>(payload) {
        return Err(ValidationError::InvalidJson(e.to_string()));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_json() {
        assert!(validate_json("{}").is_ok());
        assert!(validate_json("{\"key\": \"value\"}").is_ok());
        assert!(validate_json("[1, 2, 3]").is_ok());
    }

    #[test]
    fn test_empty_payload() {
        // 空字符串应该被拒绝
        assert!(validate_json("").is_err());
        assert!(validate_json("   ").is_err());
    }

    #[test]
    fn test_invalid_json_format() {
        // 缺少引号、缺少括号等
        assert!(validate_json("{key: value}").is_err());
        assert!(validate_json("{\"key\": }").is_err());
        assert!(validate_json("not json at all").is_err());
    }
}
