// 键盘布局数据模型
// 定义 US 104 键标准键盘的每个按键信息，包括扫描码、标签、尺寸和位置。

/// 按键的视觉状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyState {
    /// 从未被按下
    Idle,
    /// 当前正在按下
    Pressed,
    /// 已松开（曾被按过）
    Released,
}

/// 一个按键的定义
#[derive(Debug, Clone)]
pub struct KeyDef {
    /// 用于匹配的唯一标识（扫描码 + 扩展位组合）
    pub id: u32,
    /// 显示标签（主标签）
    pub label: &'static str,
    /// 副标签（如数字键盘的导航功能），None 表示无
    pub sub_label: Option<&'static str>,
    /// 按键宽度（以标准键宽 1.0u 为单位）
    pub width: f32,
    /// 按键高度（以标准键高 1.0u 为单位，大多数为 1.0）
    pub height: f32,
}

impl KeyDef {
    const fn key(id: u32, label: &'static str, width: f32) -> Self {
        Self {
            id,
            label,
            sub_label: None,
            width,
            height: 1.0,
        }
    }

    const fn key_h(id: u32, label: &'static str, width: f32, height: f32) -> Self {
        Self {
            id,
            label,
            sub_label: None,
            width,
            height,
        }
    }

    const fn key_sub(id: u32, label: &'static str, sub: &'static str, width: f32) -> Self {
        Self {
            id,
            label,
            sub_label: Some(sub),
            width,
            height: 1.0,
        }
    }
}

/// 特殊 ID 约定：
/// - 普通键：scan_code（如 Esc = 1）
/// - 扩展键：scan_code + 0x100（如 右Ctrl = 29 + 0x100 = 285）
/// - Pause：使用 0xE11D（特殊序列）

// ── 间距占位 ──────────────────────────────────────────────────

/// 占位符，用于行内间距
pub const SPACER_HALF: KeyDef = KeyDef {
    id: 0xFFFF,
    label: "",
    sub_label: None,
    width: 0.5,
    height: 1.0,
};

pub const SPACER_QUARTER: KeyDef = KeyDef {
    id: 0xFFFE,
    label: "",
    sub_label: None,
    width: 0.25,
    height: 1.0,
};

// ── 主键区 ────────────────────────────────────────────────────

/// 第 0 行：Esc + F1-F12
pub static ROW_FUNC: &[KeyDef] = &[
    KeyDef::key(0x01, "Esc", 1.0),
    SPACER_HALF,
    SPACER_HALF,
    KeyDef::key(0x3B, "F1", 1.0),
    KeyDef::key(0x3C, "F2", 1.0),
    KeyDef::key(0x3D, "F3", 1.0),
    KeyDef::key(0x3E, "F4", 1.0),
    SPACER_QUARTER,
    SPACER_QUARTER,
    KeyDef::key(0x3F, "F5", 1.0),
    KeyDef::key(0x40, "F6", 1.0),
    KeyDef::key(0x41, "F7", 1.0),
    KeyDef::key(0x42, "F8", 1.0),
    SPACER_QUARTER,
    SPACER_QUARTER,
    KeyDef::key(0x43, "F9", 1.0),
    KeyDef::key(0x44, "F10", 1.0),
    KeyDef::key(0x57, "F11", 1.0),
    KeyDef::key(0x58, "F12", 1.0),
];

/// 第 1 行：数字行（label = 上方 shifted 字符，sub_label = 下方基础字符）
pub static ROW_NUM: &[KeyDef] = &[
    KeyDef::key_sub(0x29, "~", "`", 1.0),
    KeyDef::key_sub(0x02, "!", "1", 1.0),
    KeyDef::key_sub(0x03, "@", "2", 1.0),
    KeyDef::key_sub(0x04, "#", "3", 1.0),
    KeyDef::key_sub(0x05, "$", "4", 1.0),
    KeyDef::key_sub(0x06, "%", "5", 1.0),
    KeyDef::key_sub(0x07, "^", "6", 1.0),
    KeyDef::key_sub(0x08, "&", "7", 1.0),
    KeyDef::key_sub(0x09, "*", "8", 1.0),
    KeyDef::key_sub(0x0A, "(", "9", 1.0),
    KeyDef::key_sub(0x0B, ")", "0", 1.0),
    KeyDef::key_sub(0x0C, "_", "-", 1.0),
    KeyDef::key_sub(0x0D, "+", "=", 1.0),
    KeyDef::key(0x0E, "Backspace", 2.0),
];

/// 第 2 行：Tab 行          
pub static ROW_TAB: &[KeyDef] = &[
    KeyDef::key(0x0F, "Tab", 1.5),
    KeyDef::key(0x10, "Q", 1.0),
    KeyDef::key(0x11, "W", 1.0),
    KeyDef::key(0x12, "E", 1.0),
    KeyDef::key(0x13, "R", 1.0),
    KeyDef::key(0x14, "T", 1.0),
    KeyDef::key(0x15, "Y", 1.0),
    KeyDef::key(0x16, "U", 1.0),
    KeyDef::key(0x17, "I", 1.0),
    KeyDef::key(0x18, "O", 1.0),
    KeyDef::key(0x19, "P", 1.0),
    KeyDef::key_sub(0x1A, "{", "[", 1.0),
    KeyDef::key_sub(0x1B, "}", "]", 1.0),
    KeyDef::key_sub(0x2B, "|", "\\", 1.5),
];

/// 第 3 行：CapsLock 行
pub static ROW_CAPS: &[KeyDef] = &[
    KeyDef::key(0x3A, "Caps", 1.75),
    KeyDef::key(0x1E, "A", 1.0),
    KeyDef::key(0x1F, "S", 1.0),
    KeyDef::key(0x20, "D", 1.0),
    KeyDef::key(0x21, "F", 1.0),
    KeyDef::key(0x22, "G", 1.0),
    KeyDef::key(0x23, "H", 1.0),
    KeyDef::key(0x24, "J", 1.0),
    KeyDef::key(0x25, "K", 1.0),
    KeyDef::key(0x26, "L", 1.0),
    KeyDef::key_sub(0x27, ":", ";", 1.0),
    KeyDef::key_sub(0x28, "\"", "'", 1.0),
    KeyDef::key(0x1C, "Enter", 2.25),
];

/// 第 4 行：Shift 行
pub static ROW_SHIFT: &[KeyDef] = &[
    KeyDef::key(0x2A, "L Shift", 2.25),
    KeyDef::key(0x2C, "Z", 1.0),
    KeyDef::key(0x2D, "X", 1.0),
    KeyDef::key(0x2E, "C", 1.0),
    KeyDef::key(0x2F, "V", 1.0),
    KeyDef::key(0x30, "B", 1.0),
    KeyDef::key(0x31, "N", 1.0),
    KeyDef::key(0x32, "M", 1.0),
    KeyDef::key_sub(0x33, "<", ",", 1.0),
    KeyDef::key_sub(0x34, ">", ".", 1.0),
    KeyDef::key_sub(0x35, "?", "/", 1.0),
    KeyDef::key(0x36, "R Shift", 2.75),
];

/// 第 5 行：Ctrl 行
pub static ROW_CTRL: &[KeyDef] = &[
    KeyDef::key(0x1D, "L Ctrl", 1.25),
    KeyDef::key(0x15B, "L Win", 1.25),   // 扩展键 0x5B + 0x100
    KeyDef::key(0x38, "L Alt", 1.25),
    KeyDef::key(0x39, "Space", 6.25),
    KeyDef::key(0x138, "R Alt", 1.25),   // 扩展键 0x38 + 0x100
    KeyDef::key(0x15C, "R Win", 1.25),   // 扩展键 0x5C + 0x100
    KeyDef::key(0x15D, "Menu", 1.25),    // 扩展键 0x5D + 0x100
    KeyDef::key(0x11D, "R Ctrl", 1.25),  // 扩展键 0x1D + 0x100
];

// ── 导航区 ────────────────────────────────────────────────────

/// 导航区第 0 行：PrtSc / ScrLk / Pause
pub static NAV_ROW0: &[KeyDef] = &[
    KeyDef::key(0x137, "PrtSc", 1.0),    // 扩展键
    KeyDef::key(0x46, "ScrLk", 1.0),
    KeyDef::key(0xE11D, "Pause", 1.0),   // 特殊序列
];

/// 导航区第 1 行：Ins / Home / PgUp
pub static NAV_ROW1: &[KeyDef] = &[
    KeyDef::key(0x152, "Ins", 1.0),      // 扩展键
    KeyDef::key(0x147, "Home", 1.0),
    KeyDef::key(0x149, "PgUp", 1.0),
];

/// 导航区第 2 行：Del / End / PgDn
pub static NAV_ROW2: &[KeyDef] = &[
    KeyDef::key(0x153, "Del", 1.0),
    KeyDef::key(0x14F, "End", 1.0),
    KeyDef::key(0x151, "PgDn", 1.0),
];

/// 导航区第 4 行：方向键（第 3 行为空行间距）
pub static NAV_ARROWS: &[KeyDef] = &[
    KeyDef::key(0x148, "↑", 1.0),
];

pub static NAV_ARROWS_BOTTOM: &[KeyDef] = &[
    KeyDef::key(0x14B, "←", 1.0),
    KeyDef::key(0x150, "↓", 1.0),
    KeyDef::key(0x14D, "→", 1.0),
];

// ── 数字键盘区 ────────────────────────────────────────────────

/// 数字键盘第 0 行
pub static NUM_ROW0: &[KeyDef] = &[
    KeyDef::key(0x45, "Num", 1.0),
    KeyDef::key(0x135, "/", 1.0),        // 扩展键
    KeyDef::key(0x37, "*", 1.0),
    KeyDef::key(0x4A, "-", 1.0),
];

/// 数字键盘第 1 行
pub static NUM_ROW1: &[KeyDef] = &[
    KeyDef::key_sub(0x47, "7", "Home", 1.0),
    KeyDef::key_sub(0x48, "8", "↑", 1.0),
    KeyDef::key_sub(0x49, "9", "PgUp", 1.0),
    KeyDef::key_h(0x4E, "+", 1.0, 2.0),  // + 键跨两行高
];

/// 数字键盘第 2 行
pub static NUM_ROW2: &[KeyDef] = &[
    KeyDef::key_sub(0x4B, "4", "←", 1.0),
    KeyDef::key_sub(0x4C, "5", "", 1.0),
    KeyDef::key_sub(0x4D, "6", "→", 1.0),
    // + 键在此行被上一行的 2.0 高度覆盖
];

/// 数字键盘第 3 行
pub static NUM_ROW3: &[KeyDef] = &[
    KeyDef::key_sub(0x4F, "1", "End", 1.0),
    KeyDef::key_sub(0x50, "2", "↓", 1.0),
    KeyDef::key_sub(0x51, "3", "PgDn", 1.0),
    KeyDef::key_h(0x11C, "Enter", 1.0, 2.0),  // 数字键盘 Enter，扩展键，跨两行
];

/// 数字键盘第 4 行
pub static NUM_ROW4: &[KeyDef] = &[
    KeyDef::key_sub(0x52, "0", "Ins", 2.0),   // 0 键双宽
    KeyDef::key_sub(0x53, ".", "Del", 1.0),
    // Enter 在此行被上一行的 2.0 高度覆盖
];

/// 根据钩子事件的扫描码和扩展标志计算按键 ID
pub fn make_key_id(scan_code: u32, vk_code: u32, is_extended: bool) -> u32 {
    // Pause/Break 键特殊处理：扫描码 0x45 但 vkCode == 0x13 (VK_PAUSE)
    if vk_code == 0x13 {
        return 0xE11D;
    }

    // NumLock 键：扫描码 0x45，非 Pause 时
    if scan_code == 0x45 && vk_code == 0x90 {
        return 0x45;
    }

    // 右 Shift：扫描码固定 0x36，某些驱动可能错误设置扩展标志
    if scan_code == 0x36 {
        return 0x36;
    }

    // 扩展键加 0x100 偏移
    if is_extended {
        scan_code + 0x100
    } else {
        scan_code
    }
}


