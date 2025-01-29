use std::{path::Path, sync::Mutex};
use substuff::*;
use tauri::{
    ipc::InvokeError, path, AppHandle, Emitter, Manager, WebviewUrl, WebviewWindowBuilder,
};

#[derive(Default, Debug)]
struct AppState {
    github_key: String,
    github_intermediate: GithubDeviceCodeResponse,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(AppState::default()));
            let app_handle = app.handle();
            if cfg!(debug_assertions) {
                app_handle.plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            let path_resolver = path::PathResolver::app_config_dir(app.path());
            println!("{:#?}", path_resolver);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_article,
            new_window,
            test_auth,
            start_auth,
            wait_for_auth
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Define the Tauri command
#[tauri::command]
fn get_article(file_path: String) -> Result<Article, String> {
    match get_article_from_toml_file(Path::new(&file_path)) {
        Ok(article) => Ok(article),
        Err(e) => Err(format!("Failed to parse article: {}", e)),
    }
}
#[tauri::command]
fn new_window(app: AppHandle) {
    WebviewWindowBuilder::new(
        &app,
        "window-1",
        WebviewUrl::App("https://google.com".into()),
    )
    .build()
    .unwrap();
}
#[derive(serde::Serialize, Clone)]
struct Payload {
    verification_uri: String,
    user_code: String,
}

#[tauri::command]
fn test_auth(app: AppHandle) -> Result<(), InvokeError> {
    let device_code_response = start_git_auth().map_err(|e| InvokeError::from(e.to_string()))?;

    app.emit(
        "auth-started",
        Payload {
            verification_uri: device_code_response.verification_uri.clone(),
            user_code: device_code_response.user_code.clone(),
        },
    )
    .map_err(|e| InvokeError::from(e.to_string()))?;
    println!(
        "Please go to {} and enter the code: {}",
        device_code_response.verification_uri, device_code_response.user_code
    );
    let config = GitHubConfig::new(device_code_response);
    match wait_for_github(config) {
        Ok(token) => {
            app.emit("auth-responded", token).unwrap();
            return Ok(());
        }
        Err(err) => {
            app.emit("auth-responded", String::new()).unwrap();
            return Err(InvokeError::from(format!("Error during auth: {:?}", err)));
        }
    }
}

#[tauri::command]
fn start_auth(app: AppHandle) -> Result<(), InvokeError> {
    let device_code_response = start_git_auth().map_err(|e| InvokeError::from(e.to_string()))?;
    let payload = Payload {
        verification_uri: device_code_response.verification_uri.clone(),
        user_code: device_code_response.user_code.clone(),
    };
    let state = app.state::<Mutex<AppState>>();
    let mut state = state.lock().unwrap();
    state.github_intermediate = device_code_response.clone();
    app.emit("auth-started", payload.clone())
        .map_err(|e| InvokeError::from(e.to_string()))?;
    Ok(())
}

#[tauri::command]
fn wait_for_auth(app: AppHandle) -> Result<String, InvokeError> {
    let state = app.state::<Mutex<AppState>>();
    let device_code_response = state.lock().unwrap().github_intermediate.clone();
    println!(
        "Please go to {} and enter the code: {}",
        device_code_response.verification_uri, device_code_response.user_code
    );
    let config = GitHubConfig::new(device_code_response);
    match wait_for_github(config) {
        Ok(token) => {
            app.emit("auth-responded", &token).unwrap();
            {
                let mut inner = state.lock().unwrap();
                inner.github_key = token.clone();
                println!("{:#?}", inner);
            }
            return Ok(token);
        }
        Err(err) => {
            app.emit("auth-responded", String::new()).unwrap();
            return Err(InvokeError::from(format!("Error during auth: {:?}", err)));
        }
    }
}
