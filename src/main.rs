// src/main.rs

mod entity;
mod component;

use component::register::Registry;

fn main() {
    println!("=== 极简网关测试 ===");
    
    // 假前端来要通行证了
    let my_permit = Registry::enroll_task();
    
    // 打印出来的结果
    println!("✅ 成功放行！拿到的凭证: {:#?}", my_permit);
}