//! 方向/OK：Tap 就绪时一律吞固件原生，只留 mapped 重注（防双击 wedge 豆包语音）。
//!
//!   cargo test --manifest-path src-tauri/Cargo.toml --test dpad_ok_double_fire -- --nocapture

use remote_bridge_hub_lib::bridges::xiaomi::key_mapping::{
    set_dpad_ok_custom_suppress_vks, should_gate_block_dpad_ok_mapping,
};
use remote_bridge_hub_lib::bridges::xiaomi::special_keys::should_suppress_native_dpad_ok;

#[test]
fn dpad_ok_suppressed_when_tap_ready_regardless_of_custom_list() {
    // 固件直通 extra + mapped 重注 = 双击；tap 就绪即吞，不再依赖自定义表
    set_dpad_ok_custom_suppress_vks(&[]);
    for vk in [0x25, 0x26, 0x27, 0x28, 0x0D] {
        assert!(should_suppress_native_dpad_ok(vk, true, false));
    }
    set_dpad_ok_custom_suppress_vks(&[]);
}

#[test]
fn dpad_ok_passthrough_when_tap_not_ready() {
    // Tap 未就绪：透传，遥控器退化为普通按键
    set_dpad_ok_custom_suppress_vks(&[]);
    for vk in [0x25, 0x26, 0x27, 0x28, 0x0D] {
        assert!(!should_suppress_native_dpad_ok(vk, false, false));
    }
}

#[test]
fn recent_still_suppresses() {
    set_dpad_ok_custom_suppress_vks(&[]);
    assert!(should_suppress_native_dpad_ok(0x26, false, true));
}

#[test]
fn dpad_ok_mapping_must_not_be_gate_blocked() {
    assert!(!should_gate_block_dpad_ok_mapping("up"));
    assert!(should_gate_block_dpad_ok_mapping("volume_up"));
}
