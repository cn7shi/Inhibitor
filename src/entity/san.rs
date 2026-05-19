// entity/san.rs — SAN值（污染度）数据结构与污染物 trait 定义
//
// SanSchema 是 SAN 值的运行时状态。
// Pollutant 是"能对 SAN 值造成污染的错误"的行为契约。
//
// 实现 Pollutant 分两层：
//   - 原始错误（JsonError 等）：声明固有权重（weight）
//   - 聚合错误（EntryGateError 等）：委托内部错误的 weight × 场景乘数

#[derive(Debug, Clone)]
pub struct SanSchema {
    /// 初始设定的最大SAN值（容忍度）
    pub max_san: i32,
    /// 当前剩余的SAN值（当 <= 0 时视为完全污染，触发熔断）
    pub current_san: i32,
}

impl SanSchema {
    /// 初始化一个全新的SAN状态
    pub fn new(max: i32) -> Self {
        Self {
            max_san: max,
            current_san: max,
        }
    }

    /// 判断是否已经完全被污染（触发熔断条件）
    pub fn is_corrupted(&self) -> bool {
        self.current_san <= 0
    }

    /// 扣减 SAN 值（使用预计算的扣减量）。
    /// 由 pipeline 在收集完错误后统一调用。
    pub fn apply_penalty(&mut self, penalty: i32, error_key: &str) {
        self.current_san = (self.current_san - penalty).max(0);
        tracing::warn!(
            component = "san",
            error_key = error_key,
            penalty = penalty,
            remaining = self.current_san,
            max = self.max_san,
            "SAN 扣减"
        );
    }
}

/// 污染物 trait —— 任何可能污染全局环境（扣减 SAN 值）的错误都应实现此 trait。
///
/// # 两层实现约定
/// - **原始错误**：`weight()` 返回固有权重（1~10），`error_key()` 返回自身标识
/// - **聚合错误**：`weight()` 委托给内部错误的 `weight()` 并乘以场景系数，
///   `error_key()` 委托给内部错误
pub trait Pollutant {
    /// 该错误造成的污染权重。
    /// 原始错误返回固有值（1~10），聚合错误返回经过场景系数放大后的最终值。
    fn weight(&self) -> i32;

    /// 错误的稳定标识符（如 "json::empty"），用于配置文件中的热更新覆盖匹配。
    fn error_key(&self) -> &'static str;
}
