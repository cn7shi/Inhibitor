//EntryGate.rs 数据流入口

use crate::entity::permit::Permit;
use crate::errors::ValidationError;
use crate::validator::permit::validate_permit_id;
use crate::validator::json::validate_json;
use tracing::{info, warn};

pub struct EntryGate {}

impl EntryGate {
    // 进门刷卡 (入参校验)
    pub fn check_in(permit: &Permit) -> Result<(), ValidationError> {
        validate_permit_id(permit.permit_id).inspect_err(|e| {
            warn!(gate = "entry", error = %e, "入参校验失败");
        })?;

        validate_json(&permit.payload).inspect_err(|e| {
            warn!(gate = "entry", error = %e, "入参校验失败");
        })?;

        info!(gate = "entry", permit_id = permit.permit_id, "校验通过：permit_id 合法且 JSON 格式正确");
        Ok(())
    }
}