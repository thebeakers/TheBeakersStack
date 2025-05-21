use log;
use std::{path::Path, sync::Mutex};
use substuff::*;
use tauri::{
    ipc::InvokeError, path, AppHandle, Emitter, Manager, WebviewUrl, WebviewWindowBuilder,
}; // Ensure log is imported

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

            let path_resolver = path::PathResolver::app_config_dir(app.path());
            log::info!("App config dir: {:?}", path_resolver); // Use log macro

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_article,
            new_window,
            test_auth,
            start_auth,
            wait_for_auth,
            upload_article_to_github // Added new command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_article(file_path: String) -> Result<Article, String> {
    // Ensure Article here is substuff::Article
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
    let device_code_response = start_git_auth().map_err(|e| InvokeError::from(e.to_string()))?;

    app.emit(
        "auth-started",
        Payload {
            verification_uri: device_code_response.verification_uri.clone(),
            user_code: device_code_response.user_code.clone(),
        },
    )
    .map_err(|e| InvokeError::from(e.to_string()))?;

    log::info!(
        // Use log macro
        "test_auth: Please go to {} and enter the code: {}",
        device_code_response.verification_uri,
        device_code_response.user_code
    );

    let config = GitHubConfig::new(device_code_response);
    match substuff::wait_for_github(config).await {
        Ok(token) => {
            log::info!("test_auth: Token received successfully."); // Use log macro
            app.emit("auth-responded", token)
                .map_err(|e| InvokeError::from(format!("Failed to emit auth-responded: {}", e)))?;
            Ok(())
        }
        Err(err) => {
            log::error!("test_auth: Error during GitHub polling: {:?}", err); // Use log macro
            app.emit("auth-responded", String::new()).map_err(|e| {
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
    log::info!(
        // Use log macro
        "start_auth: AppState github_intermediate updated: {:#?}",
        locked_state.github_intermediate
    );

    app.emit("auth-started", payload.clone())
        .map_err(|e| InvokeError::from(e.to_string()))?;
    Ok(payload.clone())
}

#[tauri::command]
async fn wait_for_auth(app: AppHandle) -> Result<String, InvokeError> {
    let state = app.state::<Mutex<AppState>>();
    let device_code_response = state.lock().unwrap().github_intermediate.clone();

    if device_code_response.device_code.is_empty() {
        log::error!("wait_for_auth: Error: github_intermediate not set or device_code is empty. Call start_auth first."); // Use log macro
        return Err(InvokeError::from(
            "Authentication process not started or state is invalid. Call start_auth first.",
        ));
    }

    log::info!(
        // Use log macro
        "wait_for_auth: Polling GitHub. Go to {} and enter code: {}",
        device_code_response.verification_uri,
        device_code_response.user_code
    );

    let config = GitHubConfig::new(device_code_response);
    match substuff::wait_for_github(config).await {
        Ok(token) => {
            log::info!("wait_for_auth: Token received successfully."); // Use log macro
            app.emit("auth-responded", &token)
                .map_err(|e| InvokeError::from(format!("Failed to emit auth-responded: {}", e)))?;
            {
                let mut inner_state = state.lock().unwrap();
                inner_state.github_key = token.clone();
                log::info!("wait_for_auth: AppState updated with GitHub key."); // Use log macro
            }
            println!("{}", token);
            Ok(token)
        }
        Err(err) => {
            log::error!("wait_for_auth: Error during GitHub polling: {:?}", err); // Use log macro
            app.emit("auth-responded", String::new()).map_err(|e| {
                InvokeError::from(format!("Failed to emit auth-responded (error case): {}", e))
            })?;
            Err(InvokeError::from(format!(
                "Error waiting for GitHub auth: {:?}",
                err
            )))
        }
    }
}

#[tauri::command]
async fn upload_article_to_github(
    app: AppHandle,
    article: substuff::Article, // This is substuff::Article
    file_name: String,
) -> Result<String, InvokeError> {
    log::info!("Attempting to upload article: {}", article.title);

    let github_token = {
        let state = app.state::<Mutex<AppState>>();
        let locked_state = state.lock().unwrap();
        locked_state.github_key.clone()
    };

    if github_token.is_empty() {
        log::error!("GitHub token is missing. User needs to authenticate.");
        return Err(InvokeError::from(
            "GitHub token is missing. Please authenticate first.",
        ));
    }

    log::debug!(
        "Using GitHub token (first 5 chars): {}",
        &github_token[..std::cmp::min(5, github_token.len())]
    );

    let owner = "thebeakers";
    let repo_name = "TheBeakersWebsite";
    let path_in_repo = format!("src/content/articles/{}", file_name);
    let commit_message = format!("docs: add/update article '{}' via editor", article.title);

    let toml_content = match toml::to_string_pretty(&article) {
        Ok(s) => s,
        Err(e) => {
            log::error!("Failed to serialize article to TOML: {}", e);
            return Err(InvokeError::from(format!(
                "Failed to serialize article to TOML: {}",
                e
            )));
        }
    };

    log::debug!(
        "Serialized TOML content for {}:\n{}",
        file_name,
        toml_content
    );

    match substuff::upload_file_to_github(
        &github_token,
        owner,
        repo_name,
        &path_in_repo,
        &commit_message,
        &toml_content,
    )
    .await
    {
        Ok(success_message) => {
            log::info!("Successfully uploaded article: {}", success_message);
            Ok(success_message)
        }
        Err(error_message) => {
            log::error!("Failed to upload article: {}", error_message);
            Err(InvokeError::from(error_message))
        }
    }
}
