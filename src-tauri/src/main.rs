// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use base64::encode;
use image::{DynamicImage, GenericImageView, ImageFormat};
use lazy_static::lazy_static;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, json};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::{self, create_dir_all, read_to_string, File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{exit, Command};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{env, thread};
use tauri::api::path::{app_config_dir, config_dir};
use tauri::utils::config;
use tauri::SystemTray;
use tauri::{api::process::restart, Env, Manager};
use tauri::{CustomMenuItem, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};
use tauri_plugin_autostart::MacosLauncher;
use tempfile::NamedTempFile;
use uuid::Uuid;
extern crate icns;
use icns::{IconFamily, IconType, Image};

mod handler;

// Define a static mutable variable to hold config_dir
lazy_static! {
    static ref CONFIG_DIR: Mutex<Option<PathBuf>> = Mutex::new(None);
    static ref APP_ENV: Mutex<Option<Env>> = Mutex::new(None);
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
struct BcpInitRequest {
    // this is a token from cloud -> required for cloud http request
    r#type: String,
    location_id: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
struct BcpInitResponse {
    uuid: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
struct SetBcpRequest {
    // this is a token from cloud -> required for cloud http request
    document_id: String,
    status: String,
    message: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
struct GetBcpRequest {
    // this is a token from cloud -> required for cloud http request
    location_id: String,
    r#type: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
struct LoginRequest {
    // this is a token from cloud -> required for cloud http request
    name: String,
    password: String,
    company_id: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
struct Config {
    // this is a token from cloud -> required for cloud http request
    session_id: Option<String>,
    cloud_url: Option<String>,
    client_id: Option<String>,
    version: Option<u8>,
    // api_config: Option<HashMap<String, Vec<ApiConfig>>>,
    api_config: Option<Vec<Api>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct Api {
    // this is a token from cloud -> required for cloud http request
    integration_name: Option<String>,
    icon: Option<String>,
    #[serde(rename(serialize = "isActive", deserialize = "isActive"))]
    is_active: bool,
    description: Option<String>,
    subscription_key: Option<String>,
    api_key: Option<String>,
    api: Option<Vec<Endpoint>>,
    path: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct Endpoint {
    endpoint_name: Option<String>,
    endpoint: Option<String>,
    method: Option<String>,
    header: Option<HashMap<String, String>>,
    query: Option<HashMap<String, String>>,
    body: Option<serde_json::Value>,
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
async fn fetch_data(integration_name: &str, endpoint_name: &str) -> Result<String, String> {
    println!("Debug: Calling Api endpoint ... {:#?}", endpoint_name);
    //read the local config
    let config = read_config().unwrap();
    // create bcp
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        "Authorization",
        HeaderValue::from_str(&format!(
            "Bearer {}",
            config
                .session_id
                .as_ref()
                .map(|x| x.clone())
                .unwrap_or_default()
        ))
        .unwrap(),
    );
    headers.insert(
        "Location",
        HeaderValue::from_str(
            &config
                .client_id
                .as_ref()
                .map(|x| x.clone())
                .unwrap_or_default(),
        )
        .unwrap(),
    );
    println!("debug 1");
    let req = BcpInitRequest {
        r#type: "residents_charts".to_string(),
        location_id: config
            .client_id
            .as_ref()
            .map(|x| x.clone())
            .unwrap_or_default(),
    };
    let document_id = client
        .post("http://localhost:3030/bcp/document/init")
        .headers(headers.clone())
        .json(&req)
        .send()
        .await
        .map_err(|e| format!("Failed to send response of kraken {}", e.to_string()))?
        .json::<BcpInitResponse>()
        .await
        .map_err(|e| format!("Failed to parse response of kraken{}", e.to_string()))?
        .uuid;
    println!("debug 2");
    // return to FE and spawn an other BG job

    //end.
    let config = read_config().unwrap().api_config.unwrap();
    let integration = config
        .iter()
        .find(|x| x.integration_name.clone().unwrap() == integration_name.to_string())
        .unwrap()
        .api
        .clone()
        .unwrap();
    println!("debug 3");
    let endpoint = integration
        .iter()
        .find(|x| x.endpoint_name.clone().unwrap() == endpoint_name.to_string())
        .unwrap();
    println!("debug 4");
    //activate endpoint
    match handler::send(endpoint).await {
        Ok(x) => {
            println!("debug 5");
            let req = SetBcpRequest {
                document_id,
                status: "completed".to_string(),
                message: None,
            };
            let _ = client
                .post("http://localhost:3030/bcp/document/status")
                .json(&req)
                .headers(headers.clone())
                .send()
                .await
                .map_err(|e| format!("Failed to send response of kraken {}", e.to_string()))?
                .json::<Option<i32>>()
                .await
                .map_err(|_x| "Failed to parse response of kraken".to_string())?;
            Ok(x)
        }
        Err(e) => {
            println!("debug 6");
            let req = SetBcpRequest {
                document_id,
                status: "failed".to_string(),
                message: Some(e.to_string()),
            };
            let _ = client
                .post("http://localhost:3030/bcp/document/status")
                .json(&req)
                .headers(headers.clone())
                .send()
                .await
                .map_err(|e| format!("Failed to send response of kraken {}", e.to_string()))?
                .json::<Option<i32>>()
                .await
                .map_err(|_x| "Failed to parse response of kraken".to_string())?;
            Err(e.to_string())
        }
    }
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
            // let config = tauri::Config::default();
            // println!("this is local os config for the apps: {:#?}",config);
            println!("Debug: Setting up application ...");
            // let mut config = Config::default();
            let app_name = "com.zen_blocker.dev";
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
                        client_id: None,
                        version: Some(0),
                        api_config: None,
                    };
                    println!("Debug: Creating default config file...");
                    let config_content = serde_json::to_string_pretty(&default_config)
                        .expect("Failed to serialize default config");
                    std::fs::write(&config_file_path, config_content)
                        .expect("Failed to write default config file");
                    // config = default_config;
                    // let _ = setup_dir(config_file_path, app.env());
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
                    // let _ = setup_dir(config_file_path, app.env());
                }
            }
            // set up a config_dir as mutex
            let _ = setup_dir(config_file_path, app.env());
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
                    // restart_application(&restart_mutex)
                    restart(&app.env());
                    // let window = app.get_window("main").unwrap();
                    // // window.hide().unwrap();
                    // window.show().unwrap();
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
            send_data,
            config_edit,
            fetch_app,
            block_app,
            enable_focus_mode,
            read_icns
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
async fn config_update(config: Config) -> Result<Config, String> {
    match config.session_id.as_ref() {
        Some(x) => {
            let client = reqwest::Client::new();
            let mut headers = HeaderMap::new();
            headers.insert(
                "Authorization",
                HeaderValue::from_str(&format!("Bearer {x}")).unwrap(),
            );
            // headers.insert("Location", HeaderValue::from_str(subscription_key).unwrap());
            let response = client
                .post("http://localhost:3030/update-config")
                .headers(headers)
                .json(&config)
                .send()
                .await
                .map_err(|e| format!("Failed to send response of the api {}", e.to_string()))?
                .json::<Config>()
                // .text()
                .await
                .map_err(|e| format!("Failed to parse response of the api {}", e.to_string()))?;

            let lock = CONFIG_DIR
                .lock()
                .map_err(|e| format!("Mutex lock error: {:?}", e))?;
            match &*lock {
                Some(config_dir) => {
                    // dummy_config.version = Some(config.version.unwrap_or(0) + 1);
                    println!("Debug: Updating config file...");
                    let config_content = serde_json::to_string_pretty(&response)
                        .expect("Failed to serialize default config");
                    std::fs::write(&config_dir, config_content)
                        .expect("Failed to write default config file");
                    Ok(response)
                }
                None => Err("Config directory not set".to_string()),
            }
        }
        None => Err(format!("Login required")),
    }
}

#[tauri::command]
fn config_edit(config: Config) -> Result<Config, String> {
    match config.session_id.as_ref() {
        Some(x) => {
            let lock = CONFIG_DIR
                .lock()
                .map_err(|e| format!("Mutex lock error: {:?}", e))?;
            match &*lock {
                Some(config_dir) => {
                    // config.client_id = Some("This Is Latest Config".to_string());
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
async fn login(username: &str, password: &str) -> Result<Config, String> {
    let client = reqwest::Client::new();
    let req = LoginRequest {
        name: username.to_string(),
        password: password.to_string(),
        company_id: "c1244c83-76e9-4d37-9a44-d24e46e868ac".to_string(),
    };
    let response = client
        .post("http://localhost:3030/login")
        .json(&req)
        .send()
        .await
        .map_err(|e| format!("Failed to send response of the api {}", e.to_string()))?
        .json::<Config>()
        .await
        .map_err(|_x| "Failed to parse response of the api".to_string())?;
    let _ = config_edit(response.clone());
    Ok(response)
}

#[tauri::command]
async fn fetch_app() {
    let mut app_config = read_config().unwrap();
    let mut apps = Vec::new();
    // this is macOs specific apps ... currently don't have time to play with window
    let app_dir = PathBuf::from("/Applications");
    if app_dir.exists() {
        for entry in fs::read_dir(app_dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                println!("Debug: this is apps {:?}", path);
                // apps.push(path.display().to_string());
                let name = path
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .trim()
                    .to_string();
                let path_string = path.to_string_lossy().trim().to_string();
                let icon_path_string = match find_icon_in_resources(&path_string.clone()) {
                    Some(x) => match read_icns(x) {
                        Ok(x) => Some(x),
                        _ => None,
                    },
                    None => None,
                };
        
                apps.push(Api {
                    integration_name: Some(name.clone()),
                    icon: icon_path_string,
                    is_active: false,
                    description: Some(name.replace(".app", "")),
                    subscription_key: None,
                    api_key: None,
                    api: None,
                    path: Some(path_string),
                })
            }
        }
    }
    app_config.api_config = Some(apps);
    let _ = config_edit(app_config).unwrap();
}

#[tauri::command]
fn read_icns(icns_path: PathBuf) -> Result<String, String> {
    println!("debug 0, this is icns path {:#?}", icns_path);
    // Load an icon family from an ICNS file.
    let file = match File::open(icns_path) {
        Ok(x) => x,
        Err(_e) => {
            println!("this is errror {:?}", _e.to_string());
            return Err("".to_string());
        }
    };
    let file_byte = BufReader::new(file);
    let mut icon_family = match IconFamily::read(file_byte) {
        Ok(x) => x,
        _ => return Err("".to_string()),
    };
    println!("debug 1");
    // Extract an icon from the family and save it as a PNG.
    let image_type = match icon_family.available_icons().get(0) {
        Some(x) => x.clone(),
        None => return Err("".to_string()),
    };

    let image = match icon_family.get_icon_with_type(image_type) {
        Ok(x) => x,
        _ => return Err("".to_string()),
    };
    println!("debug 2 ");
    // let file = BufWriter::new(File::create("assets/test.png").unwrap());
    // println!("debug 3");
    // image.data().write_png(file).unwrap();
    // Write the image to a temporary file.
    let mut temp_file = match NamedTempFile::new() {
        Ok(x) => x,
        _ => return Err("".to_string()),
    };
    {
        let mut buffer = Vec::new();
        match image.write_png(&mut buffer) {
            Ok(x) => (),
            _ => return Err("".to_string()),
        }
        match temp_file.write_all(&buffer) {
            Ok(x) => (),
            _ => return Err("".to_string()),
        }
    }
    match temp_file.flush() {
        Ok(x) => (),
        _ => return Err("".to_string()),
    } // Ensure the file is written
    println!("debug 4");

    // Read the temporary file and encode it to Base64.
    let mut temp_file = match File::open(temp_file.path()) {
        Ok(x) => x,
        _ => return Err("".to_string()),
    };
    let mut buffer = Vec::new();
    match temp_file.read_to_end(&mut buffer) {
        Err(_) => return Err("".to_string()),
        _ => (),
    }
    let base64_encoded = encode(&buffer);

    // // Write the Base64 string to assets/test.txt.
    // let mut output_file = OpenOptions::new()
    //     .write(true)
    //     .create(true)
    //     .open("assets/test.txt").unwrap();
    // write!(output_file, "{}", base64_encoded).unwrap();
    println!("debug 5");
    Ok(base64_encoded)
}

#[tauri::command]
async fn block_app() {
    println!("Debug: block apps triggering");
    let app_config = read_config().unwrap().api_config;

    if let Some(apps) = app_config {
        for app in apps {
            match app.is_active {
                true => {
                    if let Some(app_name) = app.integration_name {
                        // let output = match Command::new("sudo")
                        //     .arg("pkill")
                        //     .arg("-f")
                        //     .arg(app_name)
                        //     .output()
                        //     .map_err(|e| e.to_string())
                        // {
                        //     Ok(x) => {
                        //         println!("Debug: success terminate apps")
                        //     }
                        //     Err(e) => {
                        //         println!("Error: cannot block apps {:?}",e)
                        //     }
                        // };
                        let script = format!(r#"tell application "{}" to quit"#, app_name);

                        let output = match Command::new("osascript")
                            .arg("-e")
                            .arg(script)
                            .output()
                            .map_err(|e| e.to_string())
                        {
                            Ok(x) => {
                                println!("debug: sucess closing apps")
                            }
                            Err(e) => println!("Debug: error quitting apps {:?}", e),
                        };
                    }
                    continue;
                }
                false => continue,
            }
        }
    }
}

#[tauri::command]
async fn enable_focus_mode() -> Result<(), String> {
    println!("Debug: focus mode triggering 0");
    let app_config = read_config().unwrap().api_config;
    let mut app_name_vec = Vec::new();
    if let Some(apps) = app_config {
        for app in apps {
            match app.is_active {
                true => {
                    if let Some(app_name) = app.integration_name {
                        app_name_vec.push(app_name);
                    }
                    continue;
                }
                false => continue,
            }
        }
    }
    println!(
        "Debug: focus mode triggering this is app name {:?}",
        app_name_vec
    );
    // Spawn a thread to monitor and block apps
    let handle = thread::spawn(move || {
        let start_time = std::time::Instant::now();
        let duration = Duration::from_secs(20);

        while start_time.elapsed() < duration {
            println!("Debug: this is in loop ...");
            for app in &app_name_vec {
                // Check if the app is running
                let output = Command::new("pgrep")
                    .arg("-f")
                    .arg(app)
                    .output()
                    .expect("Failed to execute command");

                if !output.stdout.is_empty() {
                    println!("Debug: this is in loop, detected app running");
                    // If the app is running, quit it
                    Command::new("osascript")
                        .arg("-e")
                        .arg(format!("tell application \"{}\" to quit", app))
                        .output()
                        .expect("Failed to quit app");
                }
            }
            // Check every 5 seconds
            thread::sleep(Duration::from_secs(5));
        }
    });

    // Wait for the thread to finish
    let _ = handle.join();

    Ok(())
}

fn find_icon_in_resources(app_path: &str) -> Option<PathBuf> {
    let resources_path = Path::new(app_path).join("Contents").join("Resources");
    if !resources_path.is_dir() {
        return None;
    }
    match std::fs::read_dir(resources_path) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.extension().map_or(false, |ext| ext == "icns") {
                        println!("this is icon path {:?}", path);
                        return Some(path);
                    }
                }
            }
            None
        }
        Err(_) => None,
    }
}

// may be should read as a base64
async fn convert_icns_to_png(icns_path: PathBuf) -> Result<String, String> {
    let icns_path_str = icns_path.to_str().ok_or("Invalid ICNS path")?;

    // Create a temporary directory for the icon set
    let iconset_dir = icns_path.with_extension("iconset");
    if iconset_dir.exists() {
        fs::remove_dir_all(&iconset_dir).map_err(|e| e.to_string())?;
    }
    fs::create_dir(&iconset_dir).map_err(|e| e.to_string())?;

    // Use iconutil to convert ICNS to iconset
    let output = Command::new("iconutil")
        .arg("--convert")
        .arg("iconset")
        .arg(icns_path_str)
        .arg("--output")
        .arg(iconset_dir.to_str().ok_or("Invalid iconset path")?)
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(format!(
            "iconutil failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    // Find the largest PNG file in the iconset
    let largest_png = fs::read_dir(&iconset_dir)
        .map_err(|e| e.to_string())?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension()? == "png" {
                Some(path)
            } else {
                None
            }
        })
        .max_by_key(|path| path.metadata().ok().map(|meta| meta.len()).unwrap_or(0))
        .ok_or("No PNG files found in iconset")?;

    // Create the PNG path in the same directory as the .icns file
    let png_path = icns_path.with_extension("png");
    fs::rename(&largest_png, &png_path).map_err(|e| e.to_string())?;

    // Clean up the temporary iconset directory
    fs::remove_dir_all(&iconset_dir).map_err(|e| e.to_string())?;

    let path_string = png_path.to_string_lossy().to_string();
    Ok(path_string)
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
