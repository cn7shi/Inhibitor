// src/component/register.rs

use crate::entity::permit::Permit;

pub struct Registry {}

impl Registry {
    // 极简登记处
    pub fn enroll_task() -> Permit {
        Permit {
            permit_id: 10086,     // 随便发个 u64 编号
            permit_status: 0,     // 0 代表初始状态 (Ready)
        }
    }
}