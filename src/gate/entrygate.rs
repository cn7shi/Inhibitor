//EntryGate.rs 数据流入口

use crate::entity::permit::Permit;
use crate::errors::ValidationError;
use crate::validator::permit::validate_permit_id;
use crate::validator::json::validate_json;

pub struct EntryGate {}

impl EntryGate {
    // 进门刷卡 (入参校验)
    pub fn check_in(permit: &Permit) -> Result<(), ValidationError> {
        validate_permit_id(permit.permit_id)?;
        validate_json(&permit.payload)?;

        println!("🟢 [进门守卫] 校验通过：发现合法 permit_id ({}) 且 JSON 格式正确", permit.permit_id);
        Ok(())
    }
}