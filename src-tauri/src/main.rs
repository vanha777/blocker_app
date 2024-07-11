// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::cell::RefCell;
use std::process::{exit, Command};
use std::sync::{Arc, Mutex};
use std::{env, thread};
use tauri::SystemTray;
use tauri::{api::process::restart, Env, Manager};
use tauri::{CustomMenuItem, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};
use tauri_plugin_autostart::MacosLauncher;
// fn restart_application() {
//     Command::new(env::current_exe().unwrap())
//         .spawn()
//         .expect("Failed to restart application");
//     exit(0); // or exit(1) depending on your needs
// }

fn main() {
    println!("Debug: Starting Applcation ...");
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

    // Mutex to control restart synchronization
    let restart_mutex = Arc::new(Mutex::new(RefCell::new(())));

    // Set custom panic hook
    std::panic::set_hook(Box::new({
        let restart_mutex = restart_mutex.clone();
        move |panic_info| {
            println!("Debug: Application panicked: {:?}", panic_info);
            restart_application(&restart_mutex);
        }
    }));

    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["--flag1", "--flag2"]), /* arbitrary number of args to pass to your app */
        ))
        .setup(|app| {
            // if conditon updates detect
            // ex:
            if check_update() {
                restart(&app.env());
            }
            Ok(())
        })
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
        // .setup(|app| {
        //     if env::var("TAURI_APP_CRASHED").is_ok() {
        //         env::remove_var("TAURI_APP_CRASHED");
        //         println!("Restarting application after crash...");
        //         restart_application();
        //     }
        //     Ok(())
        // })
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            tauri::RunEvent::Exit => {
                _app_handle.restart();
            }
            // tauri::RunEvent::WindowEvent { label, event } => todo!(),
            // tauri::RunEvent::Ready => todo!(),
            // tauri::RunEvent::Resumed => todo!(),
            // tauri::RunEvent::MainEventsCleared => todo!(),
            _ => {
                // _app_handle.restart();
                // println!("Debug: Application is running");
            }
        });
    println!("Debug: Application is running");
}

fn restart_application(restart_mutex: &Arc<Mutex<RefCell<()>>>) {
    // Lock the mutex to synchronize restart
    let _lock = restart_mutex.lock().unwrap();
    // Spawn a new thread to handle the restart
    thread::spawn(move || {
        // Wait for the main thread to exit gracefully
        // Add any necessary cleanup logic here

        // Restart the application
        Command::new(env::current_exe().unwrap())
            .spawn()
            .expect("Failed to restart application");
        exit(0); // or exit(1) depending on your needs
    });
    // exit gracegully
    std::process::exit(0);
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[tauri::command]
fn check_update() -> bool {
    // updating config file ......
    println!("Debug: Sending config files to cloud and check for update ...");
    false
}

#[tauri::command]
fn crash() -> String {
    // let parsed_finger = "finger".parse::<i32>().unwrap(); // Attempt to parse and immediately unwrap
    // format!("Parsed finger: {}", parsed_finger)
    panic!("Normal panic");
}
