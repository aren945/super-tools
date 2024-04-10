// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod events;
mod modules;
use modules::tray::menu;
use tauri::{Manager, PhysicalSize, Size, SystemTray, SystemTrayMenu};
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
fn main() {
    tauri::Builder::default()
        .system_tray(menu())
        // 监听窗口事件
        .on_window_event(|event| {
            match event.event() {
                tauri::WindowEvent::Focused(isFocused) => {
                    // 如果未fouce
                    // TODO: 如果当前点击窗口外部，隐藏窗口
                    if !*isFocused {
                        // event.window().close();
                        // event.window().hide();
                    }
                }
                _ => (),
            }
            println!("event is: {:?}", event.event())
        })
        .setup(move |app| {
            let window = app.get_window("main").unwrap();
            window.set_size(Size::Physical(PhysicalSize { width: 600, height: 50 })).unwrap();
            window.center().unwrap();
            window.open_devtools();
            // 窗口最大化
            // window.maximize().unwrap();
            // 设置窗口为点击穿透
            // window.set_ignore_cursor_events(true).unwrap();
            // 创建一个异步任务来监听鼠标移动事件。
            // tauri::async_runtim::spawn(async move {
            //     // rdev::grab(move |event| {
            //     //     match event.event_type {
            //     //         // 鼠标移动事件
            //     //         EventType::MouseMove { x, y } => {
            //     //             patch_mouse_move_event(&window, (x, y));
            //     //             unsafe {
            //     //                 MOUSE_POSITION = (x, y);
            //     //             }
            //     //             false
            //     //         }
            //     //         // 鼠标移动事件
            //     //         EventType::ButtonPress(_button) => {
            //     //             unsafe {
            //     //                 patch_mousue_click_event(&window, MOUSE_POSITION);
            //     //             }
            //     //             false
            //     //         }
            //     //         _ => false,
            //     //     };
            //     //     // 返回None的话将会阻止系统的事件派发
            //     //     Some(event)
            //     // })
            //     // .unwrap();
            // });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
