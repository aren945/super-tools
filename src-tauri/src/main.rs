// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod events;
mod modules;
mod utils;
use dotenv::dotenv;
use modules::{
    tray::{handler, menu},
    window::window_event_handler,
};
use state::InitCell;
use std::{env, fs::File, io::Read, path};
use tauri::{ActivationPolicy, LogicalSize, Manager, Size};

use crate::utils::util::get_is_dev;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

static CONSTTANT: InitCell<serde_json::Value> = InitCell::new();

// 获取事件常量
fn read_event_json() -> serde_json::Value {
    // 获取当前的工作目录
    let current_dir = env::current_dir().unwrap();
    // 读取事件常量
    let constant_path = current_dir.join("../constants/events.json");
    let mut file = File::open(path::Path::new(&constant_path)).unwrap();
    let mut contenet = String::new();
    // 将读取的内容写入到contenet中
    let _ = file.read_to_string(&mut contenet);
    // 序列化成json
    let json: serde_json::Value = serde_json::from_str(&contenet).unwrap();
    CONSTTANT.set(json.clone());
    json
}

fn main() {
    // 设置环境变量
    dotenv().ok();
    // 读取事件常量
    read_event_json();
    println!("state is: {:?}", CONSTTANT.get());
    tauri::Builder::default()
        .system_tray(menu())
        .on_system_tray_event(handler)
        // 监听窗口事件
        .on_window_event(window_event_handler)
        .setup(move |app| {
            // 不创建dock应用图标
            app.set_activation_policy(ActivationPolicy::Accessory);
            let window = app.get_window("main").unwrap();
            // 使用逻辑大小来设置宽高度，不然会根据屏幕分辨率的不同呈现出不同的大小
            window.set_size(Size::Logical(LogicalSize { width: 600.0, height: 50.0 })).unwrap();
            // 如果不是开发环境，不将窗口强制置顶
            if !get_is_dev() {
                println!("23");
                window.set_always_on_top(true).unwrap();
            }
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
