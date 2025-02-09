use anyhow::{Context, Result};
use git2::StatusOptions;
use itertools::Itertools;
use std::path::Path;
use substuff::*; // Add `itertools` to Cargo.toml
fn main() {
    open_repo(
        Path::new("/tmp/gitstuff/sparce"),
        "https://github.com/thebeakers/TheBeakersStack.git",
    )
    .unwrap();
}

fn open_repo(dir_to_clone: &Path, url: &str) -> Result<()> {

    let repo = clone_or_update_repo(dir_to_clone, url).unwrap();
    let binding = repo
        .statuses(Some(&mut StatusOptions::new()))
        .context("Failed to get status")?;
    let statuses: Vec<_> = binding.into_iter().collect();
    println!("Repository status:");
    for entry in &statuses {
        if let Some(path) = entry.path() {
            let status = entry.status();
            if Path::new(path)
                .components()
                .into_iter()
                .tuple_windows::<(_, _)>()
                .map(|x| x.0.as_os_str() == "src" && x.1.as_os_str() == "articles")
                .filter(|x| *x)
                .any(|x| x)
            {
                println!("File: {} | Status: {:?}", path, status);
            }
        } else {
            println!("Unknown file entry");
        }
    }
    Ok(())
}
