// tests/gate_test.rs — gate 集成测试
// 测试完整的 gate 校验流程

use pluse::entity::permit::Permit;
use pluse::gate::entrygate::EntryGate;
use pluse::gate::exitgate::ExitGate;
use pluse::errors::ValidationError;
use std::sync::Once;

// 确保 tracing subscriber 只初始化一次（多个测试共享同一进程）
static INIT: Once = Once::new();
fn init_tracing() {
    INIT.call_once(|| {
        tracing_subscriber::fmt()
            .with_test_writer()  // 关键：配合 cargo test 的输出捕获机制
            .with_target(false)
            .init();
    });
}

/// 构造一个自定义的 Permit 用于测试
fn make_permit(id: u64, status: u8, payload: &str) -> Permit {
    Permit {
        permit_id: id,
        permit_status: status,
        payload: payload.to_string(),
    }
}

// ===== ExitGate 测试 =====

#[test]
fn exit_gate_pass_with_valid_permit() {
    init_tracing();
    let permit = make_permit(10086, 0, "{}");
    assert!(ExitGate::check_out(&permit).is_ok());
}

#[test]
fn exit_gate_reject_bad_id() {
    init_tracing();
    let permit = make_permit(99999, 0, "{}");
    let err = ExitGate::check_out(&permit).unwrap_err();
    assert_eq!(err, ValidationError::InvalidPermit(99999));
}

#[test]
fn exit_gate_reject_bad_status() {
    init_tracing();
    let permit = make_permit(10086, 3, "{}");
    let err = ExitGate::check_out(&permit).unwrap_err();
    assert_eq!(err, ValidationError::InvalidStatus(3));
}

#[test]
fn exit_gate_reject_empty_json() {
    init_tracing();
    let permit = make_permit(10086, 0, "");
    let err = ExitGate::check_out(&permit).unwrap_err();
    assert!(matches!(err, ValidationError::InvalidJson(_)));
}

#[test]
fn exit_gate_reject_broken_json() {
    init_tracing();
    let permit = make_permit(10086, 0, "{broken}");
    let err = ExitGate::check_out(&permit).unwrap_err();
    assert!(matches!(err, ValidationError::InvalidJson(_)));
}

// ===== EntryGate 测试 =====

#[test]
fn entry_gate_pass_with_valid_permit() {
    init_tracing();
    let permit = make_permit(10086, 0, "{\"result\": \"ok\"}");
    assert!(EntryGate::check_in(&permit).is_ok());
}

#[test]
fn entry_gate_reject_bad_id() {
    init_tracing();
    let permit = make_permit(0, 0, "{}");
    let err = EntryGate::check_in(&permit).unwrap_err();
    assert_eq!(err, ValidationError::InvalidPermit(0));
}

#[test]
fn entry_gate_reject_empty_json() {
    init_tracing();
    let permit = make_permit(10086, 0, "   ");
    let err = EntryGate::check_in(&permit).unwrap_err();
    assert!(matches!(err, ValidationError::InvalidJson(_)));
}
