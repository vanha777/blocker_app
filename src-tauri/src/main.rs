// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::{create_dir_all, read_to_string, File};
use std::io::Read;
use std::path::PathBuf;
use std::process::{exit, Command};
use std::sync::{Arc, Mutex};
use std::{env, thread};
use tauri::api::path::config_dir;
use tauri::SystemTray;
use tauri::{api::process::restart, Env, Manager};
use tauri::{CustomMenuItem, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};
use tauri_plugin_autostart::MacosLauncher;
use uuid::Uuid;

// Define a static mutable variable to hold config_dir
lazy_static! {
    static ref CONFIG_DIR: Mutex<Option<PathBuf>> = Mutex::new(None);
    static ref APP_ENV: Mutex<Option<Env>> = Mutex::new(None);
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Config {
    // this is a token from cloud -> required for cloud http request
    session_id: Option<String>,
    cloud_url: Option<String>,
    client_id: Option<String>,
    version: Option<u8>,
    api_config: Option<HashMap<String, Vec<ApiConfig>>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ApiConfig {
    url: Option<String>,
    method: Option<String>,
    data: Option<String>,
    active: Option<bool>,
}
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
            println!("Debug: Setting up application ...");
            let mut config = Config::default();
            let app_name = "com.strong-extractions.dev";
            let config_dir = config_dir()
                .expect("Failed to get config directory")
                .join(app_name);
            println!("Debug: Finding local config file ... {:?}", config_dir);
            // Create config_dir if not already exist
            create_dir_all(&config_dir).expect("Failed to create config directory");
            let config_file_path = config_dir.join("config.json");
            // Check if the config file exists, if not create a default one
            match !config_file_path.exists() {
                true => {
                    println!("Debug: No config file found ...");
                    let default_config = Config {
                        session_id: None,
                        cloud_url: None,
                        client_id: Some(Uuid::new_v4().to_string()),
                        version: Some(0),
                        api_config: None,
                    };
                    println!("Debug: Creating default config file...");
                    let config_content = serde_json::to_string_pretty(&default_config)
                        .expect("Failed to serialize default config");
                    std::fs::write(&config_file_path, config_content)
                        .expect("Failed to write default config file");
                    config = default_config;
                    let _ = setup_dir(config_file_path, app.env());
                }
                false => {
                    println!("Debug: Reading config file...");
                    // Read the config file
                    let mut file = File::open(&config_file_path).map_err(|e| {
                        eprintln!("Failed to open config file: {:?}", e);
                        e
                    })?;

                    let mut contents = String::new();

                    file.read_to_string(&mut contents).map_err(|e| {
                        eprintln!("Failed to read config file: {:?}", e);
                        e
                    })?;

                    let existing_config: Config = from_str(&contents).map_err(|e| {
                        eprintln!("Failed to parse config file: {:?}", e);
                        e
                    })?;
                    config = existing_config;
                    let _ = setup_dir(config_file_path, app.env());
                } // check 4 updates
                  //   if check_update(config_file_path, config) {
                  //       restart(&app.env());
                  //   }
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
        .invoke_handler(tauri::generate_handler![greet, crash, read_config, login])
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

        // // Restart the application
        // Command::new(env::current_exe().unwrap())
        //     .spawn()
        //     .expect("Failed to restart application");
        // FE request
        let lock = match APP_ENV
            .lock()
            .map_err(|e| format!("Mutex lock error: {:?}", e))
        {
            Ok(x) => x,
            Err(e) => {
                println!("Debug: Unable to restart application: {:?}", e);
                std::process::exit(0);
            }
        };
        match &*lock {
            Some(add_dir) => restart(add_dir),
            _ => {
                // // Restart the application manually
                // Command::new(env::current_exe().unwrap())
                //     .spawn()
                //     .expect("Failed to restart application");
            }
        }
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
fn check_update(config_dir: PathBuf, config: Config) -> bool {
    // updating config file ......
    println!("Debug: Sending config files to cloud and check for update ...");
    println!("Debug: No update required ...");
    config_update();
    false
}

#[tauri::command]
fn config_update() -> Result<i32, String> {
    // updating config file ......
    println!("Debug: Downloading config file ...");
    println!("Debug: Parsing config file ...");
    Ok(200)
}

#[tauri::command]
fn crash() -> String {
    // let parsed_finger = "finger".parse::<i32>().unwrap(); // Attempt to parse and immediately unwrap
    // format!("Parsed finger: {}", parsed_finger)
    panic!("Normal panic");
}

#[tauri::command]
fn read_config() -> Result<Config, String> {
    // FE request
    let lock = CONFIG_DIR
        .lock()
        .map_err(|e| format!("Mutex lock error: {:?}", e))?;
    match &*lock {
        Some(config_dir) => {
            println!("Debug: Reading config file...");
            // Read the config file
            let mut file = File::open(config_dir)
                .map_err(|e| format!("Failed to open config file: {:?}", e))?;

            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .map_err(|e| format!("Failed to open config file: {:?}", e))?;

            let existing_config: Config =
                from_str(&contents).map_err(|e| format!("Failed to open config file: {:?}", e))?;

            //check if session_id is valid

            // check 4 updates
            // under construction
            // if check_update(config_dir, existing_config) {
            //     let restart_mutex = restart_mutex.clone();
            //     move |panic_info| {
            //         println!("Debug: Application panicked: {:?}", panic_info);
            //         restart_application(&restart_mutex);
            //     }
            // }

            Ok(existing_config)
        }
        None => Err("Config directory not set".to_string()),
    }
}

#[tauri::command]
fn login(username: String, password: String) -> Result<Config, String> {
    match (username.as_str(), password.as_str()) {
        ("pharmacies1", "strongroomai") => {
            // login -> send to cloud config file

            // cloud return session_id, update require ...

            //return latest config with session_id
            let res = read_config()?;
            Ok(res)
        }
        _ => Err(format!("Invalid Credentials")),
    }
}

fn setup_dir(config_path: PathBuf, app_path: Env) -> Result<(), String> {
    let mut lock = CONFIG_DIR
        .lock()
        .map_err(|e| format!("Mutex lock error: {:?}", e))?;
    *lock = Some(config_path);
    let mut lock = APP_ENV
        .lock()
        .map_err(|e| format!("Mutex lock error: {:?}", e))?;
    *lock = Some(app_path);
    Ok(())
}
