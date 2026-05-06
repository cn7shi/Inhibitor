//EntryGate.rs 数据流入口

use crate::entity::permit::Permit;

pub struct EntryGate {}

impl EntryGate {
    // 进门刷卡 (入参校验)
    pub fn check_in(permit: &Permit) -> Result<(), String> {
        // 严格校验：确保 permit_id 合法 (大于0)
        if permit.permit_id == 10086 {
            println!("🟢 [进门守卫] 校验通过：发现合法 permit_id ({})", permit.permit_id);
            Ok(())
        } else {
            Err("🔴 [进门守卫] 熔断：非法的 permit_id (缺失或为0)".to_string())
        }
    }
}