//ExitGate.rs 数据流出口

use crate::entity::permit::Permit;
use crate::errors::ValidationError;
use crate::validator::permit::validate_permit_id;
use crate::validator::json::validate_json;

pub struct ExitGate {}

impl ExitGate {
    // 出门刷卡 (出参校验)
    pub fn check_out(permit: &Permit) -> Result<(), ValidationError> {
        if permit.permit_status != 0 {
            return Err(ValidationError::InvalidStatus(permit.permit_status));
        }

        validate_permit_id(permit.permit_id)?;
        validate_json(&permit.payload)?;

        println!("🟢 [出门守卫] 校验通过：出参合规，状态仍为 0 (Ready)，且 JSON 格式正确");
        Ok(())
    }
}
