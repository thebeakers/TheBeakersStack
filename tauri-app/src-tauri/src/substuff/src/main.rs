use anyhow::{Context, Result};
use git2::Repository;
use std::path::Path; // Add the glob dependency to your Cargo.toml.
use substuff::*;

fn main() -> Result<()> {
    let repo = clone_to_dir(Path::new("/tmp/repo"))?;
    let all_article = list_all_articles_in_toml(&repo);
    println!("{:#?}", all_article[0]);
    Ok(())
}

fn list_all_articles_in_toml(repo: &Repository) -> Vec<Article> {
    println!(
        "{}",
        repo.path()
            .parent()
            .unwrap()
            .join("Client/src/article/")
            .to_string_lossy()
    );
    std::fs::read_dir(repo.path().parent().unwrap().join("Client/src/articles/"))
        .expect("Failed to read repository directory")
        .into_iter()
        .flatten()
        .map(|x| get_article_from_toml_file(&x.path()))
        .flatten()
        .collect()
}

fn clone_to_dir(dir_to_clone: &Path) -> Result<git2::Repository> {
    let url = "https://github.com/thebeakers/TheBeakersStack.git";
    let repo = Repository::clone(url, dir_to_clone)
        .with_context(|| format!("Failed to clone repository from {}", url));
    return repo;
}
