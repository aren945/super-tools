use tauri::{Window};

#[derive(Debug, Clone, serde::Serialize)]
struct MousePosition {
  x: f64,
  y: f64
}

// 派发鼠标移动事件
pub fn patch_mouse_move_event(window: &Window, pos: (f64, f64)) {
  let is_fouced = window.is_focused();
  let _ = match is_fouced {
      Ok(value) => {
        if value {
          let _ = window.emit("mouse-position", MousePosition { x: pos.0, y: pos.1 });
        }
        ()
      },
      Err(_) => (),
  };
}

// 派发鼠标点击事件
pub fn patch_mousue_click_event(window: &Window, pos: (f64, f64)) {
  let is_fouced = window.is_focused();
  let _ = match is_fouced {
      Ok(value) => {
        if value {
          let _ = window.emit("mouse-click", MousePosition { x: pos.0, y: pos.1 });
        }
        ()
      },
      Err(_) => (),
  };
}