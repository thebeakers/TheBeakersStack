#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_article])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

use substuff::{get_article_from_toml_file, Article};

// Define the Tauri command
#[tauri::command]
fn get_article(file_path: String) -> Result<Article, String> {
    match get_article_from_toml_file(&file_path) {
        Ok(article) => Ok(article),
        Err(e) => Err(format!("Failed to parse article: {}", e)),
    }
}
