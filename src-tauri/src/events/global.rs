// 全局事件
use crate::CONSTTANT;
use tauri::Window;

pub fn patch_window_hide(window: &Window) {
    // 监听窗口失去焦点事件
    let constant = CONSTTANT.get();
    let event_name = constant["window_hide"].as_str().unwrap();
    let _ = window.emit(event_name, true);
}
