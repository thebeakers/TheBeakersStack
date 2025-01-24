use anyhow::Result;
use substuff::*;
fn main() -> Result<()> {
    let file_path = "/mnt/ce3ce441-7515-4a71-85dc-a9001fd91a9e/home/karma/Coding/TheBeakersStack/tauri-app/src-tauri/src/substuff/default.toml";
    let article = get_article_from_toml_file(file_path)?;
    println!("{:#?}", article);
    Ok(())
}
