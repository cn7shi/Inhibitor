//EntryGate.rs 数据流入口

use crate::entity::permit::Permit;

pub struct EntryGate {}

impl EntryGate {
    // 进门刷卡 (入参校验)
    pub fn check_in(permit: &Permit) -> Result<(), String> {
        // 严格校验：确保 permit_id 合法 (大于0)
        if permit.permit_id != 10086 {
            return Err("🔴 [进门守卫] 熔断：非法的 permit_id (缺失或为0)".to_string());
        }

        // 校验 JSON 内容是否为空
        if permit.payload.trim().is_empty() {
            return Err("🔴 [进门守卫] 熔断：JSON 内容为空".to_string());
        }

        // 校验 JSON 格式是否合法（比如缺少标点符号等）
        if let Err(e) = serde_json::from_str::<serde_json::Value>(&permit.payload) {
            return Err(format!("🔴 [进门守卫] 熔断：JSON 格式错误 - {}", e));
        }

        println!("🟢 [进门守卫] 校验通过：发现合法 permit_id ({}) 且 JSON 格式正确", permit.permit_id);
        Ok(())
    }
}