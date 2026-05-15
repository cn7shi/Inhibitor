//ExitGate.rs 数据流出口

use crate::entity::permit::Permit;
use crate::entity::status::Status;
use crate::errors::{GateError, StatusError};
use crate::validator::permit::validate_permit_id;
use crate::validator::json::validate_json;
use crate::validator::status::ensure_ready;
use tracing::{info, warn, error};

pub struct ExitGate {}

impl ExitGate {
    // 出门刷卡 (出参校验) — 校验通过后将状态改为 Running
    pub fn check_out(permit: &mut Permit) -> Result<(), GateError> {
        ensure_ready(permit.permit_status).inspect_err(|e| {
            warn!(gate = "exit", error = %e, "出参校验失败");
        })?;

        validate_permit_id(permit.permit_id).inspect_err(|e| {
            warn!(gate = "exit", error = %e, "出参校验失败");
        })?;

        validate_json(&permit.payload).inspect_err(|e| {
            warn!(gate = "exit", error = %e, "出参校验失败");
        })?;

        // 校验全部通过，尝试 Ready → Running
        match permit.permit_status.start() {
            Ok(new_status) => {
                permit.permit_status = new_status;
                info!(gate = "exit", permit_id = permit.permit_id, status = %permit.permit_status, "校验通过，状态已切换为 Running");
            }
            Err(msg) => {
                error!(gate = "exit", error = msg.as_str(), "状态转换异常");
                return Err(GateError::Status(StatusError::Unexpected {
                    expected: Status::Ready,
                    actual: permit.permit_status,
                }));
            }
        }
        Ok(())
    }
}
