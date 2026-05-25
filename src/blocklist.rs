// src/blocklist.rs — 统一阻塞名单（跨切面策略决策层）
//
// Blocklist 是所有"必须停下来"的终局裁决中心。
// 各组件（SAN、Gate、网络等）负责发射信号，Blocklist 负责汇总判定。
//
// 与 errors/ 的关系：
//   errors/ → 描述"出了什么问题"（可恢复，可重试）
//   blocklist → 裁决"必须停下来"（不可恢复，立即 Blocked）
//
// 职责分工：
//   validator/capability.rs → 纯扫描（"有没有命中？"）
//   blocklist.rs            → 裁决 + 执行（日志/状态/通知）

use crate::entity::permit::Permit;
use crate::component::notify::Notify;
use crate::config::Config;
use crate::validator::capability;
use tracing::error;

/// 阻塞信号 — 不可恢复的终局裁决
///
/// 每个变体代表一种独立的阻塞原因，
/// 新增阻塞条件只需在此追加变体 + 在 `enforce()` 中追加检查。
#[derive(Debug, Clone)]
pub enum BlockSignal {
    /// SAN 值归零（由多个错误累积导致，环境已被严重污染）
    SanCorrupted {
        remaining: i32,
        max: i32,
    },

    /// 能力黑名单命中（LLM 返回了危险的工具调用或内容）
    BlacklistedCapability {
        pattern: String,
        location: String, // "content" | "tool_call:<function_name>"
    },
}

impl std::fmt::Display for BlockSignal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlockSignal::SanCorrupted { remaining, max } => {
                write!(f, "SAN 归零 ({}/{}), 环境已被严重污染", remaining, max)
            }
            BlockSignal::BlacklistedCapability { pattern, location } => {
                write!(f, "能力黑名单命中：在 {} 中检测到 \"{}\"", location, pattern)
            }
        }
    }
}

/// 统一阻塞名单 — 汇总所有信号，做出最终裁决并执行善后
pub struct Blocklist;

impl Blocklist {
    /// 判定 + 执行：检查所有阻塞条件，命中则负责日志、状态变更、通知。
    ///
    /// 返回 `Ok(())` = 放行
    /// 返回 `Err(BlockSignal)` = 已阻塞（状态已变更，通知已发送）
    pub async fn enforce(permit: &mut Permit, config: &Config) -> Result<(), BlockSignal> {
        // ——— 条件 1：能力黑名单（直接一票否决，不经过 SAN）———
        if let Some(hit) = capability::scan(&permit.payload, &config.capability_blacklist) {
            let signal = BlockSignal::BlacklistedCapability {
                pattern: hit.pattern,
                location: hit.location,
            };
            Self::execute_block(permit, &signal).await;
            return Err(signal);
        }

        // ——— 条件 2：SAN 归零（由错误累积导致）———
        if permit.san.is_corrupted() {
            let signal = BlockSignal::SanCorrupted {
                remaining: permit.san.current_san,
                max: permit.san.max_san,
            };
            Self::execute_block(permit, &signal).await;
            return Err(signal);
        }

        // ——— 未来条件 3、4、5... ———

        Ok(()) // 全部通过，放行
    }

    /// 阻塞善后：日志 + 状态变更 + 通知
    async fn execute_block(permit: &mut Permit, signal: &BlockSignal) {
        error!(
            signal = %signal,
            permit_id = permit.permit_id,
            san = permit.san.current_san,
            "阻塞名单裁决"
        );

        permit.block_with_log(&signal.to_string());

        Notify::send("阻塞名单拦截", &format!(
            "permit_id: {}, 原因: {}, SAN: {}/{}",
            permit.permit_id, signal,
            permit.san.current_san, permit.san.max_san
        )).await;
    }
}
