// src/component/register.rs

use crate::entity::permit::Permit;
use crate::entity::status::Status;
use crate::entity::san::SanSchema;

pub struct Registry {}

impl Registry {
    // 极简登记处
    pub fn enroll_task() -> Permit {
        Permit {
            permit_id: 10086,
            permit_status: Status::Ready,
            payload: "{}".to_string(),
            san: SanSchema::new(100),
        }
    }
}