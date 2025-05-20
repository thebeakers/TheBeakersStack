use anyhow::{Context, Result};
use git2::{Repository, ResetType, Status};
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION},
    Client, Error as ReqwestError,
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::time::{Duration, Instant};
use std::{env, path::PathBuf};

use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
    pub url: String,
    pub alt: String,
    pub caption: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Author {
    pub name: String,
    #[serde(rename = "authorBio")]
    pub author_bio: String,
    pub slug: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Question {
    pub question: String,
    pub answers: Vec<Answer>,
    pub correct_answer: Answer,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Answer(pub String);

impl Answer {
    pub fn new(answer: String) -> Self {
        Answer(answer)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Professor {
    pub name: String,
    #[serde(rename = "professorBio")]
    pub professor_bio: String,
    pub slug: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Article {
    pub title: String,
    pub description: String,
    pub body: String,
    pub image: Image,
    pub authors: Vec<Author>,
    pub professor: Professor,
    pub questions: Option<Vec<Question>>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "publishedAt")]
    pub published_at: String,
    #[serde(rename = "readingTime")]
    pub reading_time: u32,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<String>,
    #[serde(rename = "lastUpdatedAt")]
    pub last_updated_at: Option<String>,
}

pub fn get_article_from_toml_file(file_path: &Path) -> Result<Article> {
    let toml_content = fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read the file: {}", file_path.to_str().unwrap()))?;
    let article: Article = toml::from_str(&toml_content)
        .with_context(|| "Failed to parse the TOML content into an Article struct")?;
    Ok(article)
}

#[derive(Deserialize, Debug)]
pub struct GitHubFile {
    pub name: String,
    pub path: String,
    pub r#type: String,
}

pub fn list_files_in_github_path(
    repo: &str,
    github_path: &str,
) -> Result<Vec<GitHubFile>, Box<dyn std::error::Error>> {
    let github_token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");
    let api_url = format!(
        "https://api.github.com/repos/{}/contents/{}",
        repo, github_path
    );
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("token {}", github_token))?,
    );
    headers.insert("User-Agent", HeaderValue::from_static("rust-client"));

    let client = reqwest::blocking::Client::new();
    let response = client.get(&api_url).headers(headers).send()?;

    if response.status().is_success() {
        let files: Vec<GitHubFile> = response.json()?;
        Ok(files)
    } else {
        let error_message = response.text()?;
        Err(format!("Failed to list files: {}", error_message).into())
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct GithubDeviceCodeResponse {
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    pub expires_in: u32,
    pub interval: u32,
}

pub fn start_git_auth() -> Result<GithubDeviceCodeResponse> {
    let client = reqwest::blocking::Client::new();
    let params = [
        ("client_id", "Ov23liELuI9vWwtgYC5f"),
        ("scope", "repo user"),
    ];

    let response = client
        .post("https://github.com/login/device/code")
        .header("Accept", "application/json")
        .form(&params)
        .send()?;

    if response.status().is_success() {
        let device_code_response: GithubDeviceCodeResponse = response.json()?;
        Ok(device_code_response)
    } else {
        let error_message = response.text()?;
        Err(anyhow::anyhow!(
            "Failed to get device code. Error: {}",
            error_message
        ))
    }
}

#[derive(Debug)]
pub enum OAuthError {
    AuthorizationPending,
    SlowDown,
    ExpiredToken,
    UnsupportedGrantType,
    IncorrectClientCredentials,
    IncorrectDeviceCode,
    AccessDenied,
    DeviceFlowDisabled,
    Unknown(String),
}

#[derive(Debug, Deserialize)]
pub struct OAuthResponse {
    pub access_token: Option<String>,
    pub token_type: Option<String>,
    pub scope: Option<String>,
    pub error: Option<String>,
    pub error_description: Option<String>,
}

#[derive(Debug)]
pub struct GitHubConfig {
    pub client_id: String,
    pub device_code: String,
    pub interval: u64,
    pub expires_in: u32, // <-- Added expires_in here
}

impl GitHubConfig {
    pub fn new(device_code_response: GithubDeviceCodeResponse) -> Self {
        GitHubConfig {
            client_id: "Ov23liELuI9vWwtgYC5f".to_string(),
            device_code: device_code_response.device_code,
            interval: device_code_response.interval.max(5) as u64,
            expires_in: device_code_response.expires_in, // <-- Initialize expires_in
        }
    }
}

pub async fn wait_for_github(config: GitHubConfig) -> Result<String, OAuthError> {
    let client = Client::new();
    let url = "https://github.com/login/oauth/access_token";
    let mut current_interval_secs = config.interval;
    // Now config.expires_in is available
    let deadline = Instant::now() + Duration::from_secs(config.expires_in as u64);

    while Instant::now() < deadline {
        let response_result = client
            .post(url)
            .header("Accept", "application/json")
            .form(&[
                ("client_id", &config.client_id),
                ("device_code", &config.device_code),
                (
                    "grant_type",
                    &"urn:ietf:params:oauth:grant-type:device_code".to_string(),
                ),
            ])
            .send()
            .await;

        match handle_response(response_result).await {
            Ok(Some(token)) => return Ok(token),
            Ok(None) => {
                // AuthorizationPending, continue polling
            }
            Err(OAuthError::SlowDown) => {
                current_interval_secs += 5;
            }
            Err(err) => return Err(err),
        }

        tokio::time::sleep(Duration::from_secs(current_interval_secs)).await;
    }

    Err(OAuthError::ExpiredToken)
}

async fn handle_response(
    response_result: Result<reqwest::Response, ReqwestError>,
) -> Result<Option<String>, OAuthError> {
    match response_result {
        Ok(resp) => {
            if resp.status().is_success() {
                let body: OAuthResponse = resp.json().await.map_err(|e| {
                    OAuthError::Unknown(format!("Failed to deserialize OAuthResponse: {}", e))
                })?;

                if let Some(token) = body.access_token {
                    return Ok(Some(token));
                }

                match body.error.as_deref() {
                    Some("authorization_pending") => Ok(None),
                    Some("slow_down") => Err(OAuthError::SlowDown),
                    Some("expired_token") => Err(OAuthError::ExpiredToken),
                    Some("unsupported_grant_type") => Err(OAuthError::UnsupportedGrantType),
                    Some("incorrect_client_credentials") => {
                        Err(OAuthError::IncorrectClientCredentials)
                    }
                    Some("incorrect_device_code") => Err(OAuthError::IncorrectDeviceCode),
                    Some("access_denied") => Err(OAuthError::AccessDenied),
                    Some("device_flow_disabled") => Err(OAuthError::DeviceFlowDisabled),
                    Some(other) => Err(OAuthError::Unknown(format!(
                        "GitHub error: {}. Description: {}",
                        other,
                        body.error_description.unwrap_or_else(|| "N/A".to_string())
                    ))),
                    None => Err(OAuthError::Unknown(
                        "Successful response but no token or error code.".to_string(),
                    )),
                }
            } else {
                let status = resp.status();
                let error_text = resp
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error reading response body".into());
                Err(OAuthError::Unknown(format!(
                    "HTTP Error {}: {}",
                    status, error_text
                )))
            }
        }
        Err(e) => Err(OAuthError::Unknown(format!("Network Error: {}", e))),
    }
}

pub fn ensure_repo_is_healthy(url: &str, path: &Path) -> Result<Repository> {
    match path.exists() {
        true => match path.is_dir() {
            true => match Repository::open(path) {
                Ok(x) => return Ok(x),
                Err(_) => {
                    fs::remove_dir_all(path)
                        .context("Could not delete dir")
                        .unwrap();
                    fs::create_dir(path)
                        .context("Could not create dir")
                        .unwrap();
                    Repository::clone(url, path).context("Cant Clone Repo into dir :(")
                }
            },
            false => Err(anyhow::anyhow!(
                "How is the path not a dir? Something really went wrong here please fix it"
            )),
        },
        false => {
            fs::create_dir_all(path)
                .with_context(|| "Cant Make Dir :(")
                .unwrap();
            Repository::clone(url, path).with_context(|| "Cant Clone into dir :(")
        }
    }
}
pub fn update_repo_or_reset(repo: &mut Repository) -> Result<&mut Repository, git2::Error> {
    {
        let remote_name = "origin";
        let mut remote = repo.find_remote(remote_name)?;
        let mut fetch_opts = git2::FetchOptions::new();
        let callbacks = git2::RemoteCallbacks::new();
        fetch_opts.remote_callbacks(callbacks);
        remote.fetch(
            &["refs/heads/main:refs/heads/main"],
            Some(&mut fetch_opts),
            None,
        )?;
        let remote_head = repo.find_reference("refs/remotes/origin/main")?;
        let remote_commit = remote_head.peel_to_commit()?;
        repo.reset(&remote_commit.as_object(), ResetType::Hard, None)?;
        println!(
            "Repository has been reset to the state of origin/main, discarding all local changes."
        );
    }
    Ok(repo)
}
pub fn get_changed_articles(repo: &mut Repository) -> Result<Vec<PathBuf>, git2::Error> {
    repo.statuses(None)?
        .iter()
        .filter(|entry| {
            let status = entry.status();
            status.intersects(
                Status::WT_MODIFIED
                    | Status::INDEX_NEW
                    | Status::WT_NEW
                    | Status::WT_DELETED
                    | Status::WT_RENAMED
                    | Status::WT_TYPECHANGE,
            )
        })
        .map(|entry| {
            entry
                .path()
                .map(PathBuf::from)
                .ok_or_else(|| git2::Error::from_str("Invalid path"))
        })
        .collect()
}
