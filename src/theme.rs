// 颜色主题定义

use gpui::rgb;
use gpui::Rgba;

/// 将 u32 颜色值转换为 Rgba（gpui 使用 Rgba 作为颜色类型）
fn c(hex: u32) -> Rgba {
    rgb(hex)
}

// ── 按键状态颜色 ──────────────────────────────────────────────

/// 未按下状态（默认）：深蓝灰底色
pub fn key_bg_idle() -> Rgba {
    c(0x0E1F2C)
}

/// 未按下状态：青色文字
pub fn key_fg_idle() -> Rgba {
    c(0x55C3B7)
}

/// 按下状态：橙色底色
pub fn key_bg_pressed() -> Rgba {
    c(0xFF9900)
}

/// 按下状态：深色文字
pub fn key_fg_pressed() -> Rgba {
    c(0x1A1A1A)
}

/// 已松开状态（曾被按过）：浅蓝灰底色
pub fn key_bg_released() -> Rgba {
    c(0xC8D7E3)
}

/// 已松开状态：深色文字
pub fn key_fg_released() -> Rgba {
    c(0x1A1A1A)
}

// ── 全局背景 ──────────────────────────────────────────────────

/// 窗口整体背景色
pub fn window_bg() -> Rgba {
    c(0x0A1520)
}


