// validator/capability.rs — 能力黑名单扫描（纯检查，无副作用）
//
// 扫描 LLM 响应内容，检查是否包含黑名单中的危险模式。
// 扫描目标：
//   1. choices[].message.content — 文本回复
//   2. choices[].message.tool_calls[].function.arguments — 工具调用参数
// 匹配算法还有效率问题
use tracing::warn;

/// 黑名单命中结果
pub struct BlacklistHit {
    /// 命中的模式
    pub pattern: String,
    /// 命中位置（"content" | "tool_call:<function_name>"）
    pub location: String,
}

/// 扫描 payload 中是否包含黑名单模式
///
/// 返回 `Some(BlacklistHit)` = 命中，`None` = 安全
pub fn scan(payload: &str, blacklist: &[String]) -> Option<BlacklistHit> {
    if blacklist.is_empty() {
        return None;
    }

    // 尝试解析 payload 为 JSON
    let json: serde_json::Value = match serde_json::from_str(payload) {
        Ok(v) => v,
        Err(_) => return None, // 无法解析就跳过（格式问题由 Entry Gate 处理）
    };

    let choices = json.get("choices")?.as_array()?;

    for choice in choices {
        let message = choice.get("message")?;

        // 扫描 content
        if let Some(content) = message.get("content").and_then(|v| v.as_str()) {
            if let Some(pattern) = match_blacklist(content, blacklist) {
                return Some(BlacklistHit {
                    pattern,
                    location: "content".to_string(),
                });
            }
        }

        // 扫描 tool_calls
        if let Some(tool_calls) = message.get("tool_calls").and_then(|v| v.as_array()) {
            for tc in tool_calls {
                if let Some(func) = tc.get("function") {
                    let func_name = func.get("name").and_then(|v| v.as_str()).unwrap_or("unknown");
                    let args = func.get("arguments").and_then(|v| v.as_str()).unwrap_or("");

                    if let Some(pattern) = match_blacklist(args, blacklist) {
                        return Some(BlacklistHit {
                            pattern,
                            location: format!("tool_call:{}", func_name),
                        });
                    }
                }
            }
        }
    }

    None
}

/// 将内容与黑名单模式逐一匹配（大小写不敏感），返回首个命中的模式
fn match_blacklist(content: &str, patterns: &[String]) -> Option<String> {
    let content_lower = content.to_lowercase();
    for pattern in patterns {
        if content_lower.contains(&pattern.to_lowercase()) {
            warn!(
                pattern = pattern.as_str(),
                "黑名单模式命中"
            );
            return Some(pattern.clone());
        }
    }
    None
}
