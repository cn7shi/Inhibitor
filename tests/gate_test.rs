// tests/gate_test.rs — gate 集成测试
// 测试完整的 gate 校验流程

use pluse::entity::permit::Permit;
use pluse::entity::status::Status;
use pluse::gate::entrygate::EntryGate;
use pluse::gate::exitgate::ExitGate;
use pluse::errors::ValidationError;
use std::sync::Once;

// 确保 tracing subscriber 只初始化一次（多个测试共享同一进程）
static INIT: Once = Once::new();
fn init_tracing() {
    INIT.call_once(|| {
        tracing_subscriber::fmt()
            .with_test_writer()
            .with_target(false)
            .init();
    });
}

/// 构造一个自定义的 Permit 用于测试
fn make_permit(id: u64, status: Status, payload: &str) -> Permit {
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
    let permit = make_permit(10086, Status::Ready, "{}");
    assert!(ExitGate::check_out(&permit).is_ok());
}

#[test]
fn exit_gate_reject_bad_id() {
    init_tracing();
    let permit = make_permit(99999, Status::Ready, "{}");
    let err = ExitGate::check_out(&permit).unwrap_err();
    assert_eq!(err, ValidationError::InvalidPermit(99999));
}

#[test]
fn exit_gate_reject_bad_status() {
    init_tracing();
    let permit = make_permit(10086, Status::Running, "{}");
    let err = ExitGate::check_out(&permit).unwrap_err();
    assert_eq!(err, ValidationError::InvalidStatus(Status::Running));
}

#[test]
fn exit_gate_reject_empty_json() {
    init_tracing();
    let permit = make_permit(10086, Status::Ready, "");
    let err = ExitGate::check_out(&permit).unwrap_err();
    assert!(matches!(err, ValidationError::InvalidJson(_)));
}

#[test]
fn exit_gate_reject_broken_json() {
    init_tracing();
    let permit = make_permit(10086, Status::Ready, "{broken}");
    let err = ExitGate::check_out(&permit).unwrap_err();
    assert!(matches!(err, ValidationError::InvalidJson(_)));
}

// ===== EntryGate 测试 =====

#[test]
fn entry_gate_pass_with_valid_permit() {
    init_tracing();
    let permit = make_permit(10086, Status::Ready, "{\"result\": \"ok\"}");
    assert!(EntryGate::check_in(&permit).is_ok());
}

#[test]
fn entry_gate_reject_bad_id() {
    init_tracing();
    let permit = make_permit(0, Status::Ready, "{}");
    let err = EntryGate::check_in(&permit).unwrap_err();
    assert_eq!(err, ValidationError::InvalidPermit(0));
}

#[test]
fn entry_gate_reject_empty_json() {
    init_tracing();
    let permit = make_permit(10086, Status::Ready, "   ");
    let err = EntryGate::check_in(&permit).unwrap_err();
    assert!(matches!(err, ValidationError::InvalidJson(_)));
}
