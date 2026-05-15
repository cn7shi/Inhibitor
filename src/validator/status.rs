// validator/status.rs — 状态校验器

use crate::entity::status::Status;
use crate::errors::StatusError;

/// 出门前校验：状态必须是 Ready
pub fn ensure_ready(status: Status) -> Result<(), StatusError> {
    if status != Status::Ready {
        return Err(StatusError::Unexpected {
            expected: Status::Ready,
            actual: status,
        });
    }
    Ok(())
}

/// 进门时校验：状态必须是 Running
pub fn ensure_running(status: Status) -> Result<(), StatusError> {
    if status != Status::Running {
        return Err(StatusError::Unexpected {
            expected: Status::Running,
            actual: status,
        });
    }
    Ok(())
}

