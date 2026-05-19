// validator/json.rs — JSON 原子校验器
// 每个函数只查一个具体子问题，由编排层（entry_gate / exit_gate）按需组合

use crate::errors::JsonError;

/// 检查 payload 是否为空
pub fn check_not_empty(payload: &str) -> Result<(), JsonError> {
    if payload.trim().is_empty() {
        return Err(JsonError::Empty);
    }
    Ok(())
}

/// 检查 payload 是否为合法的 JSON 格式
//此处先保留ignoreAny零拷贝写法的意见
//保留定义结构体，提取json必要信息的意见
pub fn check_format(payload: &str) -> Result<(), JsonError> {
    serde_json::from_str::<serde_json::Value>(payload)
        .map(|_| ())
        .map_err(JsonError::ParseFailed)
}
