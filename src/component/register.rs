// src/component/register.rs

use crate::entity::permit::Permit;
use crate::entity::status::Status;
use crate::entity::san::SanSchema;
use std::sync::atomic::{AtomicU64, Ordering};

/// 全局递增计数器，保证每个 Permit 拥有唯一的 permit_id
static NEXT_PERMIT_ID: AtomicU64 = AtomicU64::new(1);

pub struct Registry {}

impl Registry {
    /// 登记处：每次调用生成一个拥有专属 permit_id 的 Permit
    pub fn enroll_task() -> Permit {
        let id = NEXT_PERMIT_ID.fetch_add(1, Ordering::Relaxed);
        Permit {
            permit_id: id,
            permit_status: Status::Ready,
            payload: "{}".to_string(),
            san: SanSchema::new(100),
        }
    }
}