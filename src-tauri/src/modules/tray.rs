use tauri::{
    AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem,
};

// 托盘菜单
pub fn menu() -> SystemTray {
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("Open", "打开面板"))
        .add_item(CustomMenuItem::new("Setting", "设置"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("Exit", "退出"));
    SystemTray::new().with_menu(tray_menu)
}

// 处理托盘点击事件
pub fn handler(app: &AppHandle, event: SystemTrayEvent) {
    // 获取应用窗口
    let window = app.get_window("main").unwrap();
    match event {
        SystemTrayEvent::MenuItemClick { tray_id, id, .. } => {
            match id.as_str() {
                "Open" => {
                    // 打开主页面
                    // TODO
                    println!("Open");
                    window.show().unwrap();
                }
                "Exit" => {
                    // 退出
                    println!("Exit");
                    std::process::exit(0);
                }
                _ => (),
            }
        }
        _ => (),
    }
}
