//permit.rs
/*
    设计初衷：
    1.登记任务，进行状态初始化
    2.确认身份，防止前端传入非法参数
    3.限制任务步长，防止任务无限增生，通过扁平化达到这一目的
*/
use crate::entity::status::Status;
use crate::entity::san::SanSchema;

#[derive(Debug)]
pub struct Permit{
    pub permit_id:u64,              //任务id
    pub permit_status: Status,      //任务状态
    pub payload: String,            // 用于传输 JSON 数据
    pub san: SanSchema,             // 污染度（SAN值），生命周期跟随 Permit
}

impl Permit {
    /// 触发熔断挂起，并自动记录日志
    pub fn block_with_log(&mut self, gate: &str) {
        match self.permit_status.block() {
            Ok(blocked) => self.permit_status = blocked,
            Err(msg) => tracing::error!(error = msg.as_str(), "状态转换异常"),
        }
        tracing::error!(gate = gate, status = %self.permit_status, "校验最终失败，任务已挂起");
    }
}
