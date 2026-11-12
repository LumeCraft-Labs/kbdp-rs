// kbdp-rs — 键盘按键检测工具
// 使用 Windows 全局键盘钩子实时检测按键。
#![windows_subsystem = "windows"]

mod hook;
mod keyboard_view;
mod layout;
mod theme;

use gpui_kit::{AppContext, Bounds, WindowBounds, WindowOptions, px, size};
use keyboard_view::KeyboardView;

fn main() {
    // 启动全局键盘钩子
    let (hook_rx, _hook_handle) = hook::start_hook();

    gpui_kit::application()
        .run(move |cx| {
            gpui_kit::init(cx);

            // 窗口尺寸：容纳 104 键标准布局
            let bounds = Bounds::centered(None, size(px(1160.0), px(380.0)), cx);

            gpui_kit::open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    titlebar: Some(gpui_kit::TitlebarOptions {
                        title: Some("kbdp-rs — 键盘按键检测".into()),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                cx,
                |_, cx| cx.new(|_| KeyboardView::new(hook_rx)),
            )
            .expect("failed to open keyboard probe window");
        });
}
