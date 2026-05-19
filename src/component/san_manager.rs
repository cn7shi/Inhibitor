// component/san_manager.rs — SAN值管理器（污染度计算引擎）
//
// SanManager 是一个纯计算引擎，不认识任何具体错误，不认识任何具体场景。
// 它只做一件事：从 Pollutant 获取权重，扣减 SAN 值。
//
// 热更新支持：通过 san_overrides（来自 config.toml）覆盖特定错误的权重。
// 配置优先级：config 覆盖值 > trait 默认值

use std::collections::HashMap;
use crate::entity::san::{SanSchema, Pollutant};
use tracing::warn;

pub struct SanManager;

impl SanManager {
    /// 扣减 SAN 值。
    ///
    /// - `schema`: 当前 SAN 状态
    /// - `error`: 任何实现了 Pollutant 的错误（原始或聚合）
    /// - `overrides`: 来自 config 的热更新覆盖表（可传空 HashMap）
    ///
    /// 返回本次扣减的分值。
    pub fn deduct(
        schema: &mut SanSchema,
        error: &dyn Pollutant,
        overrides: &HashMap<String, i32>,
    ) -> i32 {
        // 优先查配置覆盖，没有则用 trait 默认值
        let penalty = overrides
            .get(error.error_key())
            .copied()
            .unwrap_or_else(|| error.weight());

        schema.current_san = (schema.current_san - penalty).max(0);

        warn!(
            component = "san",
            error_key = error.error_key(),
            penalty = penalty,
            remaining = schema.current_san,
            max = schema.max_san,
            "SAN 扣减"
        );

        penalty
    }
}

