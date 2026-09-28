//! 方向/OK：自定义映射用 Home 同款 tap_ready 吞固件 VK；身份映射不误伤真实键盘。
//! 遥控器信号由 hid_report_tap 尽早 mark，recent 窗内吞固件残留，消双触发。
//!
//!   cargo test --manifest-path src-tauri/Cargo.toml --test dpad_ok_double_fire -- --nocapture

use remote_bridge_hub_lib::bridges::xiaomi::key_mapping::{
    firmware_dpad_ok_down_age_ms, note_firmware_dpad_ok_down,
    set_dpad_ok_custom_suppress_vks, should_gate_block_dpad_ok_mapping,
    should_skip_mapped_for_firmware_win, FIRMWARE_PRE_WINDOW_MS,
};
use remote_bridge_hub_lib::bridges::xiaomi::special_keys::should_suppress_native_dpad_ok;

#[test]
fn up_mapped_to_m_suppresses_firmware_up_when_tap_ready() {
    // 与 Home→Space 相同：Tap 就绪即吞固件原生，消除空闲单点「先 M 后上」
    set_dpad_ok_custom_suppress_vks(&[0x26]);
    assert!(should_suppress_native_dpad_ok(0x26, true, false));
    // 身份左不在表内 → 真实键盘左仍可用
    assert!(!should_suppress_native_dpad_ok(0x25, true, false));
    set_dpad_ok_custom_suppress_vks(&[]);
}

#[test]
fn identity_ok_not_suppressed_on_tap_ready_alone() {
    set_dpad_ok_custom_suppress_vks(&[]);
    assert!(!should_suppress_native_dpad_ok(0x0D, true, false));
    assert!(!should_suppress_native_dpad_ok(0x25, true, false));
}

#[test]
fn recent_still_suppresses() {
    set_dpad_ok_custom_suppress_vks(&[]);
    assert!(should_suppress_native_dpad_ok(0x26, false, true));
}

#[test]
fn firmware_win_skips_mapped() {
    // 固件 DOWN 先到（100ms 内）→ 跳过重注，单次投递
    assert!(should_skip_mapped_for_firmware_win(
        Some(20),
        FIRMWARE_PRE_WINDOW_MS
    ));
    assert!(!should_skip_mapped_for_firmware_win(
        Some(500),
        FIRMWARE_PRE_WINDOW_MS
    ));
    assert!(!should_skip_mapped_for_firmware_win(
        None,
        FIRMWARE_PRE_WINDOW_MS
    ));
}

#[test]
fn firmware_note_records_recent_down() {
    note_firmware_dpad_ok_down(0x0D);
    let age = firmware_dpad_ok_down_age_ms(0x0D);
    assert!(age.is_some_and(|a| a <= FIRMWARE_PRE_WINDOW_MS));
    // 非方向/OK 不记录
    assert!(firmware_dpad_ok_down_age_ms(0x41).is_none());
}

#[test]
fn dpad_ok_mapping_must_not_be_gate_blocked() {
    assert!(!should_gate_block_dpad_ok_mapping("up"));
    assert!(should_gate_block_dpad_ok_mapping("volume_up"));
}
