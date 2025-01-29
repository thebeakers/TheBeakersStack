use anyhow::{Context, Result};
use git2::{build::CheckoutBuilder, RebaseOptions, Repository};
use reqwest::{
    blocking::{Client, Response},
    header::{HeaderMap, HeaderValue, AUTHORIZATION},
    Error,
};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::thread::sleep;
use std::time::{Duration, Instant};

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
    // Fetch the GitHub personal access token from environment variables
    let github_token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");

    // Construct the API URL for the contents endpoint
    let api_url = format!(
        "https://api.github.com/repos/{}/contents/{}",
        repo, github_path
    );

    // Set up headers
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("token {}", github_token))?,
    );
    headers.insert("User-Agent", HeaderValue::from_static("rust-client"));

    // Make the GET request
    let client = Client::new();
    let response = client.get(&api_url).headers(headers).send()?;

    // Handle response
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
    let client = Client::new();
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
    SlowDown(u64), // Extra seconds to wait
    ExpiredToken,
    UnsupportedGrantType,
    IncorrectClientCredentials,
    IncorrectDeviceCode,
    AccessDenied,
    DeviceFlowDisabled,
    Unknown(String), // Fallback for unexpected errors
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
    pub interval: u64, // Minimum polling interval in seconds
}

impl GitHubConfig {
    pub fn new(device_code_response: GithubDeviceCodeResponse) -> Self {
        GitHubConfig {
            client_id: "Ov23liELuI9vWwtgYC5f".to_string(),
            device_code: device_code_response.device_code,
            interval: device_code_response.interval as u64,
        }
    }
}

pub fn wait_for_github(config: GitHubConfig) -> Result<String, OAuthError> {
    let client = Client::new();
    let url = "https://github.com/login/oauth/access_token";
    let mut interval = config.interval;
    let deadline = Instant::now() + Duration::from_secs(900); // 15 minutes timeout
    while Instant::now() < deadline {
        let response = client
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
            .send();
        match handle_response(response) {
            Ok(Some(token)) => return Ok(token),
            Ok(None) => {
                // Continue polling
            }
            Err(OAuthError::SlowDown(extra)) => {
                interval += extra + 1;
            }
            Err(err) => return Err(err),
        }

        sleep(Duration::from_secs(interval));
    }

    Err(OAuthError::ExpiredToken) // Return expired token error if timeout reached
}

fn handle_response(response: Result<Response, Error>) -> Result<Option<String>, OAuthError> {
    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                let body: OAuthResponse = resp.json().unwrap_or_else(|_| OAuthResponse {
                    access_token: None,
                    token_type: None,
                    scope: None,
                    error: None,
                    error_description: None,
                });

                if let Some(token) = body.access_token {
                    return Ok(Some(token));
                }

                match body.error.as_deref() {
                    Some("authorization_pending") => Ok(None),
                    Some("slow_down") => Ok(None),
                    Some("expired_token") => Err(OAuthError::ExpiredToken),
                    Some("unsupported_grant_type") => Err(OAuthError::UnsupportedGrantType),
                    Some("incorrect_client_credentials") => {
                        Err(OAuthError::IncorrectClientCredentials)
                    }
                    Some("incorrect_device_code") => Err(OAuthError::IncorrectDeviceCode),
                    Some("access_denied") => Err(OAuthError::AccessDenied),
                    Some("device_flow_disabled") => Err(OAuthError::DeviceFlowDisabled),
                    Some(other) => Err(OAuthError::Unknown(other.to_string())),
                    None => Ok(None),
                }
            } else {
                Err(OAuthError::Unknown(
                    resp.text().unwrap_or_else(|_| "Unknown error".into()),
                ))
            }
        }
        Err(_) => Err(OAuthError::Unknown("Network Error".into())),
    }
}

pub fn clone_or_update_repo(dir_to_clone: &Path) -> Result<git2::Repository> {
    let url = "https://github.com/thebeakers/TheBeakersStack.git";
    if dir_to_clone.exists() && dir_to_clone.is_dir() {
        match Repository::open(dir_to_clone) {
            Ok(repo) => Ok(update_existing_repo(repo, dir_to_clone, url)?),
            Err(_) => Ok(handle_repo_clone(dir_to_clone, url)?),
        }
    } else {
        Ok(handle_repo_clone(dir_to_clone, url)?)
    }
}

fn update_existing_repo(
    repo: git2::Repository,
    dir_to_clone: &std::path::Path,
    url: &str,
) -> Result<Repository> {
    match repo.state() != git2::RepositoryState::Clean
        || repo.head().is_err()
        || !repo.statuses(None)?.is_empty()
    {
        true => {
            println!("Repository at {:?} is damaged. Re-cloning...", dir_to_clone);
            return handle_repo_clone(dir_to_clone, url);
        }
        false => {
            {
                let mut remote = repo.find_remote("origin")?;
                remote.fetch(&["main"], None, None)?;
                let fetch_commit = {
                    let fetch_head = repo.find_reference("refs/remotes/origin/main")?;
                    repo.reference_to_annotated_commit(&fetch_head)?
                };
                let mut options = RebaseOptions::default();
                let mut checkout_opts = CheckoutBuilder::new();
                checkout_opts.force(); // Overwrites any local changes unconditionally

                options.checkout_options(checkout_opts);
                let mut rebase =
                    repo.rebase(None, Some(&fetch_commit), None, Some(&mut options))?;
                while let Some(Ok(_)) = rebase.next() {}
                rebase.finish(None)?;
                println!("Repository at {:?} successfully updated.", dir_to_clone);
            }
            Ok(repo)
        }
    }
}

fn handle_repo_clone(dir_to_clone: &Path, url: &str) -> Result<git2::Repository> {
    println!("The directory exists but is not a valid git repository. Purging and cloning fresh repository...");
    fs::remove_dir_all(dir_to_clone)
        .with_context(|| format!("Failed to remove directory: {:?}", dir_to_clone))?;
    Ok(Repository::clone(url, dir_to_clone)?)
}
