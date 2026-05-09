//ExitGate.rs 数据流出口

use crate::entity::permit::Permit;
use crate::errors::ValidationError;
use crate::validator::permit::validate_permit_id;
use crate::validator::json::validate_json;
use crate::validator::status::validate_status;
use tracing::{info, warn};

pub struct ExitGate {}

impl ExitGate {
    // 出门刷卡 (出参校验)
    pub fn check_out(permit: &Permit) -> Result<(), ValidationError> {
        validate_status(permit.permit_status).inspect_err(|e| {
            warn!(gate = "exit", error = %e, "出参校验失败");
        })?;

        validate_permit_id(permit.permit_id).inspect_err(|e| {
            warn!(gate = "exit", error = %e, "出参校验失败");
        })?;

        validate_json(&permit.payload).inspect_err(|e| {
            warn!(gate = "exit", error = %e, "出参校验失败");
        })?;

        info!(gate = "exit", permit_id = permit.permit_id, "校验通过：出参合规，状态 0 (Ready)，JSON 格式正确");
        Ok(())
    }
}
