// entity/status.rs — 任务状态枚举 + 状态机转换

/// 任务在流水线中的状态
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Status {
    /// 就绪，等待执行
    Ready,
    /// 正在执行中
    Running,
    /// 被阻塞（熔断器挂起）
    Blocked,
    /// 执行完成
    Done,
    /// 执行失败
    Failed,
}

impl Status {
    /// Ready → Running（出门，开始执行）
    pub fn start(self) -> Result<Status, String> {
        match self {
            Status::Ready => Ok(Status::Running),
            other => Err(format!("非法转换：{} → Running", other)),
        }
    }

    /// 任意状态(除 Done) → Blocked（紧急制动）
    pub fn block(self) -> Result<Status, String> {
        match self {
            Status::Done => Err(format!("非法转换：Done → Blocked")),
            _ => Ok(Status::Blocked),
        }
    }

    /// Running → Done（执行完成）
    pub fn finish(self) -> Result<Status, String> {
        match self {
            Status::Running => Ok(Status::Done),
            other => Err(format!("非法转换：{} → Done", other)),
        }
    }

    /// Running → Failed（执行失败）
    pub fn fail(self) -> Result<Status, String> {
        match self {
            Status::Running => Ok(Status::Failed),
            other => Err(format!("非法转换：{} → Failed", other)),
        }
    }
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Ready => write!(f, "Ready"),
            Status::Running => write!(f, "Running"),
            Status::Blocked => write!(f, "Blocked"),
            Status::Done => write!(f, "Done"),
            Status::Failed => write!(f, "Failed"),
        }
    }
}
