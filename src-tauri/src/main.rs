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

mod handler;

// Define a static mutable variable to hold config_dir
lazy_static! {
    static ref CONFIG_DIR: Mutex<Option<PathBuf>> = Mutex::new(None);
    static ref APP_ENV: Mutex<Option<Env>> = Mutex::new(None);
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
struct Config {
    // this is a token from cloud -> required for cloud http request
    session_id: Option<String>,
    cloud_url: Option<String>,
    client_id: Option<String>,
    version: Option<u8>,
    // api_config: Option<HashMap<String, Vec<ApiConfig>>>,
    api_config: Option<Vec<serde_json::Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[tauri::command]
async fn fetch_data() -> Result<(), String> {
    let url = "https://api.fred.com.au/integrations/qat/v1/fred-office/invoices";
    let query = [("fromDate", "2024-06-01"), ("toDate", "2024-06-30")];
    let subscription_key = "963f415e031a4b32a4a1915e26e085ca";
    let fred_api_key = "MGND9YRNVC/m+7RAoLmoBgUo1lwI+jfCggyPTcUILDZhYtjJJ9fWr2sITM1BLcMpjsqpxV/mGf98lVvdn8HBsLs7nzFecYPV/B7eY9ONu+5pg2r2Ki0UYz0Z7S4JjP7BYNMEDgpCzyC37C3fbosUF8wwi7nYAQhg1OKNiPgqwwgSIVJKuhD9k/DKYEX0QDXuU=";

    handler::send_query(url, &query, subscription_key, fred_api_key)
        .await
        .map_err(|e| format!("Error fetching data: {:?}", e))
}

#[tauri::command]
async fn send_data() -> Result<(), String> {
    let url = "http://127.0.0.1:5173/upload";
    let subscription_key = "your_subscription_key";
    let fred_api_key = "your_fred_api_key";
    let dummy_payload = r#"{"key1": "value1", "key2": "value2"}"#;

    handler::send_dummy_data(url, subscription_key, fred_api_key, dummy_payload)
        .await
        .map_err(|e| format!("Error sending data: {:?}", e))
}

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
            // let mut config = Config::default();
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
                    // config = default_config;
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

                    // let existing_config: Config = from_str(&contents).map_err(|e| {
                    //     eprintln!("Failed to parse config file: {:?}", e);
                    //     e
                    // })?;
                    // config = existing_config;
                    let _ = setup_dir(config_file_path, app.env());
                }
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
        .invoke_handler(tauri::generate_handler![
            greet,
            crash,
            read_config,
            login,
            config_update,
            fetch_data,
            send_data
        ])
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
        let lock = match APP_ENV
            .lock()
            .map_err(|e| format!("Mutex lock error: {:?}", e))
        {
            Ok(x) => x,
            Err(_e) => {
                // // Restart the application manually
                Command::new(env::current_exe().unwrap())
                    .spawn()
                    .expect("Failed to restart application");
                exit(0);
            }
        };
        match &*lock {
            Some(add_dir) =>
            // Restart with apps env
            {
                restart(add_dir)
            }
            _ => {
                // // Restart the application manually
                Command::new(env::current_exe().unwrap())
                    .spawn()
                    .expect("Failed to restart application");
            }
        }
        exit(0); // or exit(1) depending on your needs
    });

    // main thread exit gracegully
    std::process::exit(0);
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

// #[tauri::command]
// fn check_update(config: Config) -> Result<Config, String> {
//     // updating config file ......
//     match config_update(config.clone()) {
//         Ok(x) => {
//             println!("Debug: Sending config files to cloud and check for update ...");
//             Ok(x)
//         }
//         Err(e) => Err(format!("Failed to update")),
//     }
// }

#[tauri::command]
fn config_update(config: Config) -> Result<Config, String> {
    match config.session_id.as_ref() {
        Some(x) => {
            //send to cloud check session_id for permission
            // .....
            //save to local file
            // modify session_id and save to local file
            let lock = CONFIG_DIR
                .lock()
                .map_err(|e| format!("Mutex lock error: {:?}", e))?;
            match &*lock {
                Some(config_dir) => {
                    println!("Debug: Updating config file...");
                    let config_content = serde_json::to_string_pretty(&config)
                        .expect("Failed to serialize default config");
                    std::fs::write(&config_dir, config_content)
                        .expect("Failed to write default config file");
                    Ok(config)
                }
                None => Err("Config directory not set".to_string()),
            }
        }
        None => Err(format!("Login required")),
    }
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

            Ok(existing_config)
        }
        None => Err("Config directory not set".to_string()),
    }
}

#[tauri::command]
fn login(username: &str, password: &str) -> Result<Config, String> {
    match (username, password) {
        ("pharmacies1", "strongroomai") => {
            //return latest config with session_id
            // this is just randomly -> should be return from cloud
            let session_id = Uuid::new_v4().to_string();
            let mut res = read_config()?;
            res.session_id = Some(session_id);
            res.version = Some(1);
            res.cloud_url = Some("http://127.0.0.1:5173".to_string());
            // modify session_id and save to local file
            let lock = CONFIG_DIR
                .lock()
                .map_err(|e| format!("Mutex lock error: {:?}", e))?;
            match &*lock {
                Some(config_dir) => {
                    println!("Debug: Updating config file...");
                    let config_content = serde_json::to_string_pretty(&res)
                        .expect("Failed to serialize default config");
                    std::fs::write(&config_dir, config_content)
                        .expect("Failed to write default config file");
                    Ok(res)
                }
                None => Err("Config directory not set".to_string()),
            }
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
