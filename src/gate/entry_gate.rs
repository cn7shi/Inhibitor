// gate/entrygate.rs 数据流入口

use crate::entity::permit::Permit;
use crate::errors::EntryGateError;
use crate::validator::entry_gate;
use tracing::{info, warn};

pub struct EntryGate {}

impl EntryGate {
    // 进门刷卡 — 确认工具带着结果安全回到 agent
    pub fn check_in(permit: &Permit) -> Result<(), Vec<EntryGateError>> {
        entry_gate::run(permit).inspect_err(|errors| {
            for e in errors {
                warn!(gate = "entry", error = %e, "入参校验失败");
            }
        })?;

        info!(gate = "entry", permit_id = permit.permit_id, "校验通过");
        Ok(())
    }
}