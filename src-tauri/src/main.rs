// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::env;
use std::process::{exit, Command};
use tauri::SystemTray;
use tauri::{api::process::restart, Env, Manager};
use tauri::{CustomMenuItem, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};
fn restart_application() {
    Command::new(env::current_exe().unwrap())
        .spawn()
        .expect("Failed to restart application");
    exit(0); // or exit(1) depending on your needs
}
fn main() {
    // here `"quit".to_string()` defines the menu item id, and the second parameter is the menu item label.
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    // let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let open = CustomMenuItem::new("open".to_string(), "Open");
    let update = CustomMenuItem::new("update".to_string(), "Update");
    let tray_menu = SystemTrayMenu::new()
        .add_item(open)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(update)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);
    let tray = SystemTray::new().with_menu(tray_menu);

    // Set up a panic hook to catch panics and restart the application
    std::panic::set_hook(Box::new(|panic_info| {
        println!("Application panicked: {:?}", panic_info);
        env::set_var("TAURI_APP_CRASHED", "true");
    }));

    tauri::Builder::default()
        .system_tray(tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick {
                position: _,
                size: _,
                ..
            } => {
                println!("system tray received a left click");
            }
            SystemTrayEvent::RightClick {
                position: _,
                size: _,
                ..
            } => {
                println!("system tray received a right click");
            }
            SystemTrayEvent::DoubleClick {
                position: _,
                size: _,
                ..
            } => {
                println!("system tray received a double click");
            }
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                "open" => {
                    let window = app.get_window("main").unwrap();
                    // window.hide().unwrap();
                    window.show().unwrap();
                }
                "update" => {
                    let window = app.get_window("main").unwrap();
                    // window.hide().unwrap();
                    window.show().unwrap();
                }
                _ => {}
            },
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![greet, crash])
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        // set up restart apps with path when crashes
        // .setup(|app| {
        //     restart(&app.env());
        //     Ok(())
        // })
        .setup(|app| {
            if env::var("TAURI_APP_CRASHED").is_ok() {
                env::remove_var("TAURI_APP_CRASHED");
                println!("Restarting application after crash...");
                restart_application();
            }
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        });
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[tauri::command]
fn crash() -> String {
    let parsed_finger = "finger".parse::<i32>().unwrap(); // Attempt to parse and immediately unwrap
    format!("Parsed finger: {}", parsed_finger)
}
