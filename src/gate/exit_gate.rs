// gate/exitgate.rs 数据流出口

use crate::entity::permit::Permit;
use crate::entity::status::Status;
use crate::errors::{ExitGateError, StatusError};
use crate::validator::exit_gate;
use tracing::{info, warn, error};

pub struct ExitGate {}

impl ExitGate {
    // 出门刷卡 — 校验通过后将状态切换为 Running
    pub fn check_out(permit: &mut Permit) -> Result<(), Vec<ExitGateError>> {
        exit_gate::run(permit).inspect_err(|errors| {
            for e in errors {
                warn!(gate = "exit", error = %e, "出参校验失败");
            }
        })?;

        // 校验全部通过，尝试 Ready → Running
        match permit.permit_status.start() {
            Ok(new_status) => {
                permit.permit_status = new_status;
                info!(gate = "exit", permit_id = permit.permit_id, status = %permit.permit_status, "校验通过，状态已切换为 Running");
            }
            Err(msg) => {
                error!(gate = "exit", error = msg.as_str(), "状态转换异常");
                return Err(vec![ExitGateError::Status(StatusError::Unexpected {
                    expected: Status::Ready,
                    actual: permit.permit_status,
                })]);
            }
        }
        Ok(())
    }
}
