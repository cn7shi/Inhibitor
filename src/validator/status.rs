// validator/status.rs — 状态校验器

use crate::entity::status::Status;
use crate::errors::ValidationError;

/// 出门前校验：状态必须是 Ready
pub fn ensure_ready(status: Status) -> Result<(), ValidationError> {
    if status != Status::Ready {
        return Err(ValidationError::InvalidStatus(status));
    }
    Ok(())
}

/// 进门时校验：状态必须是 Running
pub fn ensure_running(status: Status) -> Result<(), ValidationError> {
    if status != Status::Running {
        return Err(ValidationError::InvalidStatus(status));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ensure_ready_pass() {
        assert!(ensure_ready(Status::Ready).is_ok());
    }

    #[test]
    fn test_ensure_ready_reject_running() {
        assert!(ensure_ready(Status::Running).is_err());
    }

    #[test]
    fn test_ensure_running_pass() {
        assert!(ensure_running(Status::Running).is_ok());
    }

    #[test]
    fn test_ensure_running_reject_ready() {
        assert!(ensure_running(Status::Ready).is_err());
    }
}
