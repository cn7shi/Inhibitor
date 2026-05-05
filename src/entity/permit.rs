//permit.rs
/*
    设计初衷：
    1.登记任务，进行状态初始化
    2.确认身份，防止前端传入非法参数
    3.限制任务步长，防止任务无限增生，通过扁平化达到这一目的
*/
#[derive(Debug)]
pub struct Permit{
    pub permit_id:u64,   //任务id
    pub permit_status:u8,//任务状态
}
