//ExitGate.rs 数据流出口

use crate::entity::permit::Permit;
use crate::errors::ValidationError;
use crate::validator::permit::validate_permit_id;
use crate::validator::json::validate_json;
use crate::validator::status::ensure_ready;
use tracing::{info, warn};

pub struct ExitGate {}

impl ExitGate {
    // 出门刷卡 (出参校验) — 校验通过后将状态改为 Running
    pub fn check_out(permit: &mut Permit) -> Result<(), ValidationError> {
        ensure_ready(permit.permit_status).inspect_err(|e| {
            warn!(gate = "exit", error = %e, "出参校验失败");
        })?;

        validate_permit_id(permit.permit_id).inspect_err(|e| {
            warn!(gate = "exit", error = %e, "出参校验失败");
        })?;

        validate_json(&permit.payload).inspect_err(|e| {
            warn!(gate = "exit", error = %e, "出参校验失败");
        })?;

        // 校验全部通过，状态从 Ready → Running
        permit.permit_status = permit.permit_status.start()
            .expect("状态转换失败：无法从当前状态切换到 Running");
        info!(gate = "exit", permit_id = permit.permit_id, status = %permit.permit_status, "校验通过，状态已切换为 Running");
        Ok(())
    }
}
