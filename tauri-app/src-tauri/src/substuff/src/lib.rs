use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;

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

pub fn get_article_from_toml_file(file_path: &str) -> Result<Article> {
    let toml_content = fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read the file: {}", file_path))?;
    let article: Article = toml::from_str(&toml_content)
        .with_context(|| "Failed to parse the TOML content into an Article struct")?;
    Ok(article)
}
