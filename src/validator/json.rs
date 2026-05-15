// validator/json.rs — JSON 格式校验器
// 只管校验对错，不管校验出什么结果
use crate::errors::JsonError;

/// 校验 JSON 字符串是否合法
/// 1. 不能为空
/// 2. 必须是合法的 JSON 格式
pub fn validate_json(payload: &str) -> Result<(), JsonError> {
    if payload.trim().is_empty() {
        return Err(JsonError::Empty);
    }

    serde_json::from_str::<serde_json::Value>(payload)
        .map(|_| ())
        .map_err(JsonError::ParseFailed)
}


