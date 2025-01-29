use anyhow::{Context, Result};
use git2::{Cred, FetchOptions, RemoteCallbacks, Repository, StatusOptions};
use itertools::Itertools;
use std::fs;
use std::path::Path;
use substuff::*; // Add `itertools` to Cargo.toml
fn main() {
    // let dir = Path::new("/tmp/gitstuff/repo");
    // match clone_or_update_repo(dir) {
    //     Ok(_) => println!("Operation completed successfully."),
    //     Err(e) => eprintln!("Operation failed: {}", e),
    // }

    handle_repo_clone(
        Path::new("/tmp/gitstuff/sparce"),
        "https://github.com/thebeakers/TheBeakersStack.git",
    )
    .unwrap();
}

use std::process::Command;

fn handle_repo_clone(dir_to_clone: &Path, url: &str) -> Result<()> {
    let repo_dir = dir_to_clone.to_str().unwrap();

    // // Clone with --filter=none and --no-checkout
    // Command::new("git")
    //     .args(["clone", "--filter=blob:none", "--sparse", url, repo_dir])
    //     .status()
    //     .context("Failed to clone repository")?;

    // // Change directory into cloned repo and enable sparse checkout
    // Command::new("git")
    //     .args([
    //         "-C",
    //         repo_dir,
    //         "sparse-checkout",
    //         "set",
    //         "--no-cone",
    //         "Client/src/articles",
    //     ])
    //     .status()
    //     .context("Failed to add Client/src/articles to sparse checkout")?;

    let repo = Repository::open(dir_to_clone)?;
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
