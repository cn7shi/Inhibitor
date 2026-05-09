//ExitGate.rs 数据流出口

use crate::entity::permit::Permit;

pub struct ExitGate {}

impl ExitGate {
    // 出门刷卡 (出参校验)
    pub fn check_out(permit: &Permit) -> Result<(), String> {
        // 严格校验：有 ID，且 status 为 0
        if permit.permit_id != 10086 || permit.permit_status != 0 {
            return Err(format!(
                "🔴 [出门守卫] 熔断：出参异常 (ID: {}, Status: {})",
                permit.permit_id, permit.permit_status
            ));
        }

        // 校验 JSON 内容是否为空
        if permit.payload.trim().is_empty() {
            return Err("🔴 [出门守卫] 熔断：JSON 内容为空".to_string());
        }

        // 校验 JSON 格式是否合法（比如缺少标点符号等）
        if let Err(e) = serde_json::from_str::<serde_json::Value>(&permit.payload) {
            return Err(format!("🔴 [出门守卫] 熔断：JSON 格式错误 - {}", e));
        }

        println!("🟢 [出门守卫] 校验通过：出参合规，状态仍为 0 (Ready)，且 JSON 格式正确");
        Ok(())
    }
}
