use anyhow::{Context, Result};
use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine as _};
use git2::{Repository, ResetType, Status};
use log; // Added for logging
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION},
    Client, Error as ReqwestError, StatusCode,
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
    pub answers: Vec<String>, // MODIFIED: Was Vec<Answer>
    #[serde(rename = "correct_answer")]
    pub correct_answer: String, // MODIFIED: Was Answer
}

// REMOVED Answer struct as it's no longer needed
// #[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
// pub struct Answer(pub String);
//
// impl Answer {
//     pub fn new(answer: String) -> Self {
//         Answer(answer)
//     }
// }

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
    pub category: String, // ADDED category field
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
    repo_name_with_owner: &str, // Changed to "owner/repo" format
    github_path: &str,
) -> Result<Vec<GitHubFile>, Box<dyn std::error::Error>> {
    let github_token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");
    let api_url = format!(
        "https://api.github.com/repos/{}/contents/{}",
        repo_name_with_owner, github_path
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
    pub expires_in: u32,
}

impl GitHubConfig {
    pub fn new(device_code_response: GithubDeviceCodeResponse) -> Self {
        GitHubConfig {
            client_id: "Ov23liELuI9vWwtgYC5f".to_string(),
            device_code: device_code_response.device_code,
            interval: device_code_response.interval.max(5) as u64,
            expires_in: device_code_response.expires_in,
        }
    }
}

pub async fn wait_for_github(config: GitHubConfig) -> Result<String, OAuthError> {
    let client = Client::new();
    let url = "https://github.com/login/oauth/access_token";
    let mut current_interval_secs = config.interval;
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

// Structs for GitHub API interaction (file upload)
#[derive(Serialize, Debug)]
struct GitHubPutFileRequest<'a> {
    message: &'a str,
    content: String, // Base64 encoded
    sha: Option<String>,
    committer: Committer<'a>,
}

#[derive(Serialize, Debug)]
struct Committer<'a> {
    name: &'a str,
    email: &'a str,
}

#[derive(Deserialize, Debug)]
struct GitHubFileGetResponse {
    sha: String,
}

#[derive(Deserialize, Debug)]
struct GitHubErrorResponse {
    message: String,
}

#[derive(Deserialize, Debug)]
struct GitHubPutFileResponse {
    commit: GitHubCommitInfo,
}

#[derive(Deserialize, Debug)]
struct GitHubCommitInfo {
    sha: String,
    message: String,
}

pub async fn upload_file_to_github(
    token: &str,
    owner: &str,
    repo: &str,
    path_in_repo: &str,
    commit_message: &str,
    file_content: &str,
) -> Result<String, String> {
    let client = Client::new();
    let api_url = format!(
        "https://api.github.com/repos/{}/{}/contents/{}",
        owner, repo, path_in_repo
    );

    log::debug!("Attempting to GET file info from: {}", api_url);
    let get_response = client
        .get(&api_url)
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("User-Agent", "TauriProfessorApp/0.1.0")
        .send()
        .await;

    let mut current_sha: Option<String> = None;

    match get_response {
        Ok(resp) => {
            log::debug!(
                "GET response status for {}: {}",
                path_in_repo,
                resp.status()
            );
            if resp.status() == StatusCode::OK {
                match resp.json::<GitHubFileGetResponse>().await {
                    Ok(file_data) => {
                        current_sha = Some(file_data.sha);
                        log::info!(
                            "File {} exists, SHA: {}",
                            path_in_repo,
                            current_sha.as_ref().unwrap()
                        );
                    }
                    Err(e) => {
                        log::warn!(
                            "Failed to parse GET response for {}: {}. Assuming new file.",
                            path_in_repo,
                            e
                        );
                    }
                }
            } else if resp.status() == StatusCode::NOT_FOUND {
                log::info!("File {} not found. Will create a new file.", path_in_repo);
            } else {
                let status = resp.status();
                let error_text = resp
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error reading response body".into());
                log::error!(
                    "Failed to get file info for {}: {} - {}",
                    path_in_repo,
                    status,
                    error_text
                );
                return Err(format!(
                    "GitHub API error (GET {}): {} - {}",
                    path_in_repo, status, error_text
                ));
            }
        }
        Err(e) => {
            log::error!(
                "Network error while getting file info for {}: {}",
                path_in_repo,
                e
            );
            return Err(format!("Network error (GET {}): {}", path_in_repo, e));
        }
    }

    let encoded_content = BASE64_STANDARD.encode(file_content.as_bytes());

    let request_body = GitHubPutFileRequest {
        message: commit_message,
        content: encoded_content,
        sha: current_sha.clone(),
        committer: Committer {
            name: "Professor App",       // Using a fixed committer name
            email: "app@thebeakers.com", // Using a fixed committer email
        },
    };

    log::debug!("Preparing to PUT file to: {}", api_url);
    log::debug!(
        "PUT request body: message='{}', sha='{:?}'",
        request_body.message,
        request_body.sha
    );

    let put_response = client
        .put(&api_url)
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("User-Agent", "TauriProfessorApp/0.1.0")
        .json(&request_body)
        .send()
        .await;

    match put_response {
        Ok(resp) => {
            let status = resp.status();
            log::debug!("PUT response status for {}: {}", path_in_repo, status);
            if status == StatusCode::OK || status == StatusCode::CREATED {
                match resp.json::<GitHubPutFileResponse>().await {
                    Ok(put_file_response) => {
                        let action = if current_sha.is_some() {
                            "updated"
                        } else {
                            "created"
                        };
                        let success_msg = format!(
                            "File {} successfully {} in {}/{}. Commit SHA: {}",
                            path_in_repo, action, owner, repo, put_file_response.commit.sha
                        );
                        log::info!("{}", success_msg);
                        Ok(success_msg)
                    }
                    Err(e) => {
                        log::error!(
                            "Failed to parse successful PUT response for {}: {}",
                            path_in_repo,
                            e
                        );
                        Err(format!(
                            "Failed to parse GitHub API response (PUT {}): {}",
                            path_in_repo, e
                        ))
                    }
                }
            } else {
                let error_body_text = resp
                    .text()
                    .await
                    .unwrap_or_else(|_| "Could not read error response body".to_string());
                log::error!(
                    "Raw error response body for PUT {}: {}",
                    path_in_repo,
                    error_body_text
                );
                let error_message = serde_json::from_str::<GitHubErrorResponse>(&error_body_text)
                    .map(|gh_error| gh_error.message)
                    .unwrap_or_else(|_| error_body_text);

                log::error!(
                    "Failed to upload file {}: {} - {}",
                    path_in_repo,
                    status,
                    error_message
                );
                Err(format!(
                    "GitHub API error (PUT {}): {} - {}",
                    path_in_repo, status, error_message
                ))
            }
        }
        Err(e) => {
            log::error!("Network error while uploading file {}: {}", path_in_repo, e);
            Err(format!("Network error (PUT {}): {}", path_in_repo, e))
        }
    }
}
