// 全局键盘钩子模块
// 使用 Windows WH_KEYBOARD_LL 低级键盘钩子捕获所有按键事件，
// 通过 channel 将事件传递给 UI 线程。
// 当本程序窗口在前台时，拦截大部分系统快捷键（Alt+F4 除外）。

use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

use windows::Win32::Foundation::{LPARAM, LRESULT, WPARAM};
use windows::Win32::System::Threading::GetCurrentProcessId;
use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, DispatchMessageW, GetForegroundWindow, GetMessageW,
    GetWindowThreadProcessId, SetWindowsHookExW, UnhookWindowsHookEx,
    KBDLLHOOKSTRUCT, MSG, WH_KEYBOARD_LL, WM_KEYDOWN, WM_KEYUP, WM_SYSKEYDOWN,
    WM_SYSKEYUP,
};

/// 钩子线程发送给 UI 线程的按键事件
#[derive(Debug, Clone, Copy)]
pub struct HookKeyEvent {
    /// 硬件扫描码
    pub scan_code: u32,
    /// Windows 虚拟键码
    pub vk_code: u32,
    /// true = 按下，false = 松开
    pub is_pressed: bool,
    /// 是否为扩展键（右侧 Ctrl、Alt、方向键、Ins/Del 等）
    pub is_extended: bool,
}

// 线程局部存储：钩子回调中用于发送事件
thread_local! {
    static HOOK_SENDER: std::cell::RefCell<Option<Sender<HookKeyEvent>>> = const { std::cell::RefCell::new(None) };
}

/// VK_F4
const VK_F4: u32 = 0x73;
/// LLKHF_ALTDOWN 标志位
const LLKHF_ALTDOWN: u32 = 0x20;

/// 检查当前前台窗口是否属于本进程
fn is_our_window_foreground() -> bool {
    unsafe {
        let fg = GetForegroundWindow();
        if fg.is_invalid() {
            return false;
        }
        let mut pid: u32 = 0;
        GetWindowThreadProcessId(fg, Some(&mut pid));
        pid == GetCurrentProcessId()
    }
}

/// 判断是否为 Alt+F4 组合（必须放行，否则用户无法关闭窗口）
fn is_alt_f4(kb: &KBDLLHOOKSTRUCT) -> bool {
    kb.vkCode == VK_F4 && (kb.flags.0 & LLKHF_ALTDOWN) != 0
}

/// 低级键盘钩子的回调函数
unsafe extern "system" fn keyboard_proc(
    n_code: i32,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    if n_code >= 0 {
        let kb = unsafe { &*(l_param.0 as *const KBDLLHOOKSTRUCT) };
        let msg = w_param.0 as u32;

        let is_pressed = msg == WM_KEYDOWN || msg == WM_SYSKEYDOWN;
        let is_released = msg == WM_KEYUP || msg == WM_SYSKEYUP;

        if is_pressed || is_released {
            // LLKHF_EXTENDED = 0x01，表示扩展键
            let is_extended = (kb.flags.0 & 0x01) != 0;

            let event = HookKeyEvent {
                scan_code: kb.scanCode,
                vk_code: kb.vkCode,
                is_pressed,
                is_extended,
            };

            HOOK_SENDER.with(|sender| {
                if let Some(ref tx) = *sender.borrow() {
                    let _ = tx.send(event);
                }
            });

            // 本程序窗口在前台时拦截按键，防止触发系统快捷键
            // Alt+F4 例外，必须放行以便用户关闭窗口
            if is_our_window_foreground() && !is_alt_f4(kb) {
                return LRESULT(1);
            }
        }
    }

    // 非本程序前台时放行所有按键
    unsafe { CallNextHookEx(None, n_code, w_param, l_param) }
}

/// 钩子句柄，用于退出时清理
pub struct HookHandle {
    thread: Option<thread::JoinHandle<()>>,
}

impl Drop for HookHandle {
    fn drop(&mut self) {
        // 线程在消息循环结束后自然退出
        // 此处等待线程结束（实际上需要 PostThreadMessage WM_QUIT 来中断消息循环）
        // 简单实现：不主动等待，线程会在进程退出时终止
        let _ = self.thread.take();
    }
}

/// 启动全局键盘钩子，返回事件接收端和钩子句柄。
/// 钩子在独立线程中运行 Windows 消息循环。
pub fn start_hook() -> (Receiver<HookKeyEvent>, HookHandle) {
    let (tx, rx) = mpsc::channel::<HookKeyEvent>();

    let handle = thread::spawn(move || {
        // 将 sender 存入线程局部存储，供钩子回调使用
        HOOK_SENDER.with(|sender| {
            *sender.borrow_mut() = Some(tx);
        });

        // 安装低级键盘钩子
        let hook = unsafe {
            SetWindowsHookExW(WH_KEYBOARD_LL, Some(keyboard_proc), None, 0)
        };

        let hook = match hook {
            Ok(h) => h,
            Err(e) => {
                log::error!("安装键盘钩子失败: {e}");
                return;
            }
        };

        // 运行消息循环（低级钩子要求调用线程有消息循环）
        unsafe {
            let mut msg = MSG::default();
            while GetMessageW(&mut msg, None, 0, 0).as_bool() {
                let _ = DispatchMessageW(&msg);
            }
        }

        // 清理钩子
        let _ = unsafe { UnhookWindowsHookEx(hook) };

        HOOK_SENDER.with(|sender| {
            *sender.borrow_mut() = None;
        });
    });

    (rx, HookHandle { thread: Some(handle) })
}
