// use std::path::Path;
// use substuff::*;
// fn main() {
//     let mut repo = ensure_repo_is_healthy(
//         "https://github.com/thebeakers/TheBeakersWebsite.git",
//         &Path::new("/tmp/gitstuff/sparce"),
//     )
//     .unwrap();
//     // let repo = update_repo_or_reset(&mut repo).unwrap();
//     // thread::sleep(Duration::from_secs(10));
//     let everything_changed = get_changed_articles(&mut repo).unwrap();
//     println!("{:#?}", everything_changed)
// }
// fn update_file_via_api(pac_token: String, file_path: &Path) {}

use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Serialize, Deserialize)]
struct FileContent {
    message: String,
    content: String,
    sha: Option<String>, // SHA of the current file version (for updates)
}

async fn update_file_via_api(
    pac_token: String,
    file_path: &Path,
    repo_link: String,
) -> Result<(), Error> {
    let repo_owner = "thebeakers"; // Replace with the actual owner of the repo
    let repo_name = "TheBeakersWebsite"; // Replace with the actual repository name
    let file_path_str = file_path.to_str().unwrap();

    // Set up the client
    let client = Client::new();

    // GitHub API endpoint for updating the file
    let api_url = format!(
        "https://api.github.com/repos/{}/{}/contents/{}",
        repo_owner, repo_name, file_path_str
    );

    // Fetch the current file data to get the SHA
    let get_response = client
        .get(&api_url)
        .header("Authorization", format!("token {}", pac_token))
        .header("User-Agent", "Rust-Client")
        .send()
        .await?;

    let current_file: FileContent = get_response.json().await?;
    let current_sha = &current_file.sha.unwrap_or_default();

    // Prepare the new content to update
    let new_content = base64::encode("New file content goes here"); // Update the content here
    let message = "Updating file content via API";

    let update_content = FileContent {
        message: message.to_string(),
        content: new_content,
        sha: Some(current_sha.to_string()),
    };

    // Send the PUT request to update the file
    let response = client
        .put(&api_url)
        .header("Authorization", format!("token {}", pac_token))
        .header("User-Agent", "Rust-Client")
        .json(&update_content)
        .send()
        .await?;

    if response.status().is_success() {
        println!("File updated successfully!");
    } else {
        eprintln!("Failed to update file: {:?}", response.status());
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    let pac_token = "your_github_token_here ".to_string();
    let file_path = Path::new("path/to/your/file.txt");
    let repo_link = "https://github.com/thebeakers/TheBeakersWebsite.git".to_string();

    if let Err(e) = update_file_via_api(pac_token, file_path, repo_link).await {
        eprintln!("Error updating file: {:?}", e);
    }
}
