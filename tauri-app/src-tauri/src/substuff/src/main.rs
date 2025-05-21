use anyhow::Result;
use substuff::upload_file_to_github; // Import the function

// Configure these for your test environment
const GITHUB_TOKEN: &str = ""; // !!! REPLACE WITH YOUR TOKEN !!!
const TEST_OWNER: &str = "thebeakers"; // Replace with your GitHub username or organization
const TEST_REPO: &str = "testing-push"; // Replace with your test repository name
const PATH_IN_REPO: &str = "src/articles/article_from_cli.toml"; // Path within the repo

#[tokio::main]
async fn main() -> Result<()> {
    if GITHUB_TOKEN == "YOUR_GITHUB_TOKEN_HERE" {
        log::error!("Please replace 'YOUR_GITHUB_TOKEN_HERE' with your actual GitHub PAT in src-tauri/src/substuff/src/main.rs");
        return Ok(());
    }

    log::info!(
        "Attempting to upload/update '{}' in repository '{}/{}'",
        PATH_IN_REPO,
        TEST_OWNER,
        TEST_REPO
    );

    // Sample TOML content for an article
    // This should match the structure of your substuff::Article
    let article_toml_content = r#"
title = "Test Article from Rust CLI"
description = "This is a test article uploaded via a Rust CLI program."
body = "<p>Hello from Rust! This content has been updated by the CLI test.</p>"
category = "cli-testing"
createdAt = "2025-05-21T14:30:00.000Z"
publishedAt = "2025-05-21T14:35:00.000Z"
updatedAt = "2025-05-21T14:40:00.000Z" # Example of an updated date
lastUpdatedAt = "" # Can be empty if not applicable
readingTime = 3

[image]
url = "https://placehold.co/600x400/aabbcc/ffffff?text=Test+Image+CLI"
alt = "Test Image Alt from CLI"
caption = "Test Image Caption from CLI"

[[authors]]
name = "Rust CLI Test Script"
authorBio = "An automated author from a Rust test script."
slug = "rust-cli-author"

[professor]
name = "Dr. Rust CLI"
professorBio = "Overseeing Rust CLI tests."
slug = "dr-rust-cli"

[[questions]]
question = "Did this test upload work?"
answers = ["Yes, it did!", "No, it failed.", "I'm not sure."]
correct_answer = "Yes, it did!"
"#;

    let commit_message = format!(
        "feat: add/update test article via CLI - {}",
        chrono::Utc::now()
    );

    match upload_file_to_github(
        GITHUB_TOKEN,
        TEST_OWNER,
        TEST_REPO,
        PATH_IN_REPO,
        &commit_message,
        article_toml_content,
    )
    .await
    {
        Ok(success_message) => {
            log::info!("Successfully processed file: {}", success_message);
        }
        Err(error_message) => {
            log::error!("Failed to process file: {}", error_message);
        }
    }

    Ok(())
}
