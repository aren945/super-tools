use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu};

// 托盘菜单
pub fn menu() -> SystemTray {
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("Open", "打开面板"))
        .add_item(CustomMenuItem::new("Setting", "设置"));
    SystemTray::new().with_menu(tray_menu)
}
