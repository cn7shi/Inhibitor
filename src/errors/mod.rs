// errors/mod.rs — 错误类型统一入口
// 每种错误类型独立在自己的模块里，按需引用

pub mod gate;
pub mod json;
pub mod permit;
pub mod status;

// 便捷 re-export，外部可直接 use crate::errors::JsonError 等
pub use gate::GateError;
pub use json::JsonError;
pub use permit::PermitError;
pub use status::StatusError;
