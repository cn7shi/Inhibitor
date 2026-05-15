//! errors/mod.rs — 错误类型统一入口
//!
//! 这里是整个 AI Runtime 中间件的错误处理中心。
//! 每种错误类型独立在自己的模块里，按需引用。
//!
//! # 架构设计
//! - 采用了 `thiserror` 来简化自定义错误的书写。
//! - 现在可以更快捷地自定义错误，只需在子模块追加枚举变体即可。
//!
//! ## 错误分层
//! - **原始错误**（`permit` / `json` / `status`）：描述具体的失败原因
//! - **聚合错误**（`entry_gate` / `exit_gate`）：收拢各自 Gate 的子错误，两者独立演化
//!
//! ## 快捷导出
//! 外部模块不需要关心具体路径，可直接 `use crate::errors::EntryGateError;`

pub mod entry_gate;
pub mod exit_gate;
pub mod json;
pub mod permit;
pub mod status;

pub use entry_gate::EntryGateError;
pub use exit_gate::ExitGateError;
pub use json::JsonError;
pub use permit::PermitError;
pub use status::StatusError;
