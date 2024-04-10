use tauri::GlobalWindowEvent;

use crate::events::global::patch_window_hide;

pub fn window_event_handler(event: GlobalWindowEvent) {
    match event.event() {
        tauri::WindowEvent::Focused(is_focused) => {
            // 如果未fouce
            if !*is_focused {
                event.window().hide().unwrap();
                patch_window_hide(event.window())
            }
        }
        _ => (),
    }
}
