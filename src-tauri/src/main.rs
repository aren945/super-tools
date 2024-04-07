// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod events;

use events::mouse::{patch_mouse_move_event, patch_mousue_click_event};
use rdev::EventType;
use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// 声明一个全局变量，用于存储鼠标的位置
static mut MOUSE_POSITION: (f64, f64) = (0.0, 0.0);

fn main() {
    tauri::Builder::default()
        .setup(move |app| {
            let window = app.get_window("main").unwrap();
            // 窗口最大化
            window.maximize().unwrap();
            // 设置窗口为点击穿透
            window.set_ignore_cursor_events(true).unwrap();
            // 创建一个异步任务来监听鼠标移动事件。
            tauri::async_runtime::spawn(async move {
                rdev::grab(move |event| {
                    match event.event_type {
                        // 鼠标移动事件
                        EventType::MouseMove { x, y } => {
                            patch_mouse_move_event(&window, (x, y));
                            unsafe {
                                MOUSE_POSITION = (x, y);
                            }
                            false
                        }
                        // 鼠标移动事件
                        EventType::ButtonPress(_button) => {
                            unsafe {
                                patch_mousue_click_event(&window, MOUSE_POSITION);
                            }
                            false
                        }
                        _ => false,
                    };
                    // 返回None的话将会阻止系统的事件派发
                    Some(event)
                })
                .unwrap();
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
