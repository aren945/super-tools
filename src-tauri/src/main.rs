// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod events;

use events::mouse::{patch_mouse_move_event, patch_mousue_click_event};
use rdev::{Button, EventType};
use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// 声明一个全局变量，用于存储鼠标的位置
static mut MOUSE_POSITION:(f64, f64) = (0.0, 0.0);

fn main() {
    tauri::Builder::default()
        .setup(move |app| {
            let window = app.get_window("main").unwrap();
            let monitor = window.current_monitor().unwrap();
            match monitor {
                Some(mo) => {
                    println!("m is: {:?}", mo.size())
                }
                None => (),
            }
            //     .set_ignore_cursor_events(true)
            //     .expect("设置鼠标穿透错误");
            // 创建一个异步任务来监听鼠标移动事件。
            tauri::async_runtime::spawn(async move {
                rdev::grab(move |event| {
                    match event.event_type {
                        EventType::MouseMove { x, y } => {
                            patch_mouse_move_event(&window,(x, y));
                            unsafe {
                                MOUSE_POSITION = (x, y);
                            }
                            false
                        }
                        EventType::ButtonPress(button) => {
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
