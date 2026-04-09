// 键盘可视化渲染模块

use std::collections::HashMap;
use std::sync::mpsc::Receiver;

use gpui::{
    Context, IntoElement, ParentElement, Render, SharedString, Styled, Window,
    div, px,
};

use crate::hook::HookKeyEvent;
use crate::layout::{self, KeyDef, KeyState};
use crate::theme;

/// 区域间距比例（占一个标准键宽的比例）
const SECTION_GAP_RATIO: f32 = 0.35;
/// 按键间距比例（每个按键四周各留 GAP_RATIO/2 的空白）
const GAP_RATIO: f32 = 0.08;
/// 按键圆角比例
const RADIUS_RATIO: f32 = 0.08;
/// 主标签字体比例
const FONT_RATIO: f32 = 0.23;
/// 副标签字体比例
const SUB_FONT_RATIO: f32 = 0.18;
/// 外边距（像素）
const PADDING: f32 = 12.0;

/// 键盘总宽度（单位 u）：主键区 15u + 导航区 3u + 小键盘 4u
const TOTAL_WIDTH_U: f32 = 15.0 + 3.0 + 4.0;
/// 键盘总高度（单位 u）：6 行
const TOTAL_HEIGHT_U: f32 = 6.0;

pub struct KeyboardView {
    /// 按键 ID → 当前状态
    key_states: HashMap<u32, KeyState>,
    /// 来自钩子线程的事件接收端
    hook_rx: Receiver<HookKeyEvent>,
}

impl KeyboardView {
    pub fn new(hook_rx: Receiver<HookKeyEvent>) -> Self {
        Self {
            key_states: HashMap::new(),
            hook_rx,
        }
    }

    /// 从 channel 接收所有待处理的按键事件，更新内部状态
    fn poll_events(&mut self) {
        while let Ok(event) = self.hook_rx.try_recv() {
            let id = layout::make_key_id(event.scan_code, event.vk_code, event.is_extended);
            let state = if event.is_pressed {
                KeyState::Pressed
            } else {
                KeyState::Released
            };
            self.key_states.insert(id, state);
        }
    }

    /// 获取某个按键的当前状态
    fn state_of(&self, id: u32) -> KeyState {
        self.key_states.get(&id).copied().unwrap_or(KeyState::Idle)
    }

    /// 渲染单个按键为固定宽度的"单元格"。
    /// 单元格宽 = key.width * u，高 = key.height * u，
    /// 按键内容在单元格内缩进 gap/2 实现间距，无需 flexbox gap。
    fn render_key(&self, key: &KeyDef, u: f32) -> impl IntoElement {
        let gap = u * GAP_RATIO;
        let cell_w = key.width * u;
        let cell_h = key.height * u;

        // 占位符（间距）：仅占宽度，不渲染内容
        if key.id >= 0xFFFE {
            return div()
                .w(px(cell_w))
                .h(px(cell_h))
                .into_any_element();
        }

        let state = self.state_of(key.id);
        let (bg, fg) = match state {
            KeyState::Idle => (theme::key_bg_idle(), theme::key_fg_idle()),
            KeyState::Pressed => (theme::key_bg_pressed(), theme::key_fg_pressed()),
            KeyState::Released => (theme::key_bg_released(), theme::key_fg_released()),
        };

        let key_w = cell_w - gap;
        let key_h = cell_h - gap;
        let radius = u * RADIUS_RATIO;
        let font = u * FONT_RATIO;
        let sub_font = u * SUB_FONT_RATIO;
        let half_gap = gap / 2.0;

        let label: SharedString = key.label.into();

        let mut key_el = div()
            .w(px(key_w))
            .h(px(key_h))
            .bg(bg)
            .rounded(px(radius))
            .flex()
            .flex_col()
            .justify_center()
            .items_center()
            .overflow_hidden()
            .text_color(fg)
            .text_size(px(font))
            .child(label);

        // 副标签（shifted 字符的基础字符、数字键盘的导航功能）
        if let Some(sub) = key.sub_label {
            if !sub.is_empty() {
                let sub_label: SharedString = sub.into();
                key_el = key_el.child(
                    div()
                        .text_size(px(sub_font))
                        .text_color(fg)
                        .child(sub_label),
                );
            }
        }

        // 外层单元格：固定尺寸，内含 half_gap 的内边距使按键居中
        div()
            .w(px(cell_w))
            .h(px(cell_h))
            .pt(px(half_gap))
            .pl(px(half_gap))
            .child(key_el)
            .into_any_element()
    }

    /// 渲染一行按键（无 flexbox gap，间距内含在单元格中）
    fn render_row(&self, keys: &[KeyDef], u: f32) -> impl IntoElement {
        let mut row = div().flex().flex_row();
        for key in keys {
            row = row.child(self.render_key(key, u));
        }
        row
    }

    /// 渲染主键区（左侧 6 行）
    fn render_main_section(&self, u: f32) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .child(self.render_row(layout::ROW_FUNC, u))
            .child(self.render_row(layout::ROW_NUM, u))
            .child(self.render_row(layout::ROW_TAB, u))
            .child(self.render_row(layout::ROW_CAPS, u))
            .child(self.render_row(layout::ROW_SHIFT, u))
            .child(self.render_row(layout::ROW_CTRL, u))
    }

    /// 渲染导航区（中间区域）
    /// PrtSc 对齐 F 键行，Ins 对齐数字行，方向键对齐 Shift/Ctrl 行
    fn render_nav_section(&self, u: f32) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .child(self.render_row(layout::NAV_ROW0, u))   // PrtSc/ScrLk/Pause → F 键行
            .child(self.render_row(layout::NAV_ROW1, u))   // Ins/Home/PgUp → 数字行
            .child(self.render_row(layout::NAV_ROW2, u))   // Del/End/PgDn → Tab 行
            .child(div().h(px(u)))                         // 空行 → Caps 行
            // 方向键区：上键居中 → Shift 行
            .child(
                div()
                    .w(px(3.0 * u))
                    .flex()
                    .flex_row()
                    .justify_center()
                    .child(self.render_key(&layout::NAV_ARROWS[0], u)),
            )
            .child(self.render_row(layout::NAV_ARROWS_BOTTOM, u)) // ←↓→ → Ctrl 行
    }

    /// 渲染数字键盘区（右侧）
    fn render_numpad_section(&self, u: f32) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .child(div().h(px(u))) // 空出 F 键行，让 NumLock 对齐数字行
            .child(self.render_row(layout::NUM_ROW0, u))
            // 第 1-2 行：7/8/9 和 4/5/6，右侧 + 键跨两行
            .child(
                div().flex().flex_row().child(
                    div()
                        .flex()
                        .flex_col()
                        .child(self.render_row(&layout::NUM_ROW1[..3], u))
                        .child(self.render_row(layout::NUM_ROW2, u)),
                ).child(
                    self.render_key(&layout::NUM_ROW1[3], u),
                ),
            )
            // 第 3-4 行：1/2/3 和 0/.，右侧 Enter 跨两行
            .child(
                div().flex().flex_row().child(
                    div()
                        .flex()
                        .flex_col()
                        .child(self.render_row(&layout::NUM_ROW3[..3], u))
                        .child(self.render_row(layout::NUM_ROW4, u)),
                ).child(
                    self.render_key(&layout::NUM_ROW3[3], u),
                ),
            )
    }
}

impl Render for KeyboardView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // 每次渲染前从 channel 拉取最新事件
        self.poll_events();

        // 安排下一帧继续刷新（约 60fps）
        cx.spawn(async |entity, cx| {
            cx.background_executor()
                .timer(std::time::Duration::from_millis(16))
                .await;
            let _ = entity.update(cx, |_view, cx| {
                cx.notify();
            });
        })
        .detach();

        // 根据窗口视口大小动态计算按键单位尺寸
        let viewport = window.viewport_size();
        let vw: f32 = viewport.width.into();
        let vh: f32 = viewport.height.into();
        let section_gaps = 2.0 * SECTION_GAP_RATIO;
        let u_by_w = (vw - PADDING * 2.0) / (TOTAL_WIDTH_U + section_gaps);
        let u_by_h = (vh - PADDING * 2.0) / TOTAL_HEIGHT_U;
        let u = u_by_w.min(u_by_h);
        let section_gap = u * SECTION_GAP_RATIO;

        div()
            .flex()
            .flex_row()
            .bg(theme::window_bg())
            .size_full()
            .p(px(PADDING))
            .gap(px(section_gap))
            .justify_center()
            .items_center()
            .child(self.render_main_section(u))
            .child(self.render_nav_section(u))
            .child(self.render_numpad_section(u))
    }
}
