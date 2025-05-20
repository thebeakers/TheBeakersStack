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
            test_auth, // Will become async
            start_auth,
            wait_for_auth // Will become async
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

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
async fn test_auth(app: AppHandle) -> Result<(), InvokeError> {
    // Now async
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
        "test_auth: Please go to {} and enter the code: {}",
        device_code_response.verification_uri, device_code_response.user_code
    );

    let config = GitHubConfig::new(device_code_response);
    match substuff::wait_for_github(config).await {
        // Await the async call
        Ok(token) => {
            println!("test_auth: Token received successfully.");
            app.emit("auth-responded", token)
                .map_err(|e| InvokeError::from(format!("Failed to emit auth-responded: {}", e)))?;
            Ok(())
        }
        Err(err) => {
            eprintln!("test_auth: Error during GitHub polling: {:?}", err);
            app.emit("auth-responded", String::new()) // Emit empty on error
                .map_err(|e| {
                    InvokeError::from(format!("Failed to emit auth-responded (error case): {}", e))
                })?;
            Err(InvokeError::from(format!("Error during auth: {:?}", err)))
        }
    }
}

#[tauri::command]
fn start_auth(app: AppHandle) -> Result<Payload, InvokeError> {
    let device_code_response = start_git_auth().map_err(|e| InvokeError::from(e.to_string()))?;
    let payload = Payload {
        verification_uri: device_code_response.verification_uri.clone(),
        user_code: device_code_response.user_code.clone(),
    };
    let state = app.state::<Mutex<AppState>>();
    let mut locked_state = state.lock().unwrap();
    locked_state.github_intermediate = device_code_response.clone();
    // Log state change
    println!(
        "start_auth: AppState github_intermediate updated: {:#?}",
        locked_state.github_intermediate
    );

    app.emit("auth-started", payload.clone())
        .map_err(|e| InvokeError::from(e.to_string()))?;
    Ok(payload.clone())
}

#[tauri::command]
async fn wait_for_auth(app: AppHandle) -> Result<String, InvokeError> {
    // Now async
    let state = app.state::<Mutex<AppState>>();
    let device_code_response = state.lock().unwrap().github_intermediate.clone();

    if device_code_response.device_code.is_empty() {
        eprintln!("wait_for_auth: Error: github_intermediate not set or device_code is empty. Call start_auth first.");
        return Err(InvokeError::from(
            "Authentication process not started or state is invalid. Call start_auth first.",
        ));
    }

    println!(
        "wait_for_auth: Polling GitHub. Go to {} and enter code: {}",
        device_code_response.verification_uri, device_code_response.user_code
    );

    let config = GitHubConfig::new(device_code_response);
    match substuff::wait_for_github(config).await {
        // Await the async call
        Ok(token) => {
            println!("wait_for_auth: Token received successfully.");
            app.emit("auth-responded", &token)
                .map_err(|e| InvokeError::from(format!("Failed to emit auth-responded: {}", e)))?;
            {
                let mut inner_state = state.lock().unwrap();
                inner_state.github_key = token.clone();
                println!("wait_for_auth: AppState updated with GitHub key.");
            }
            Ok(token)
        }
        Err(err) => {
            eprintln!("wait_for_auth: Error during GitHub polling: {:?}", err);
            app.emit("auth-responded", String::new()) // Emit empty on error
                .map_err(|e| {
                    InvokeError::from(format!("Failed to emit auth-responded (error case): {}", e))
                })?;
            Err(InvokeError::from(format!(
                "Error waiting for GitHub auth: {:?}",
                err
            )))
        }
    }
}
