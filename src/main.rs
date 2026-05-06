// src/main.rs

mod entity;
mod component;


use component::register::Registry;
use component::entrygate::EntryGate;
use component::exitgate::ExitGate;



fn main() {
    println!("=== 极简网关测试 ===");
    
    //任务开始  测试加mut
    let  my_permit = Registry::enroll_task();
    
    // 打印Permit
    println!("[内部登记] 拿到的凭证: {:#?}", my_permit);
    // my_permit.permit_id = 10087;

    /* 
        2.出门安检 (Exit)：准备离开agemt，去调用外部工具 （拿着刚初始化的 permit，检查状态是否干净）
    */
   
    if let Err(e) = ExitGate::check_out(&my_permit) {
        println!("{}", e);
        /*
           待补充决策     
        */
        return; // 状态不对，不准出门！
    }

    // 3. 核心执行 (Execute)：工具在外面辛勤工作...
    println!("[外部执行] 离开agent，正在调用外部工具 (同步模拟)...");
    

    // 4. 进门安检 (Entry)：工具带着结果回来了，准备进入agent
    // （查验带回来的 permit_id 是否合法）
    // my_permit.permit_id = 10087;

    if let Err(e) = EntryGate::check_in(&my_permit) {
        println!("{}", e);
        /*
           待补充决策     
        */
        return; // 查验不合格，拦截在门外！
    }

    println!("[主流程] 恭喜！工具执行完毕，数据安全回到agent，进入下一轮思考。");



    
}
//执行流水线
/*
    2026-5-6  
    enroll->exit->execute->entry->execute...
    
    任务先在内部注册，随后，经过出参校验后，开始调用模型API，
    API返回调用结果，再经过入参校验，取得调用结果。
*/
