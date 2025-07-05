use crate::llm::generate_readme;
use crate::rag::collect_code_chunks;
use std::path::PathBuf;
use std::fs::write;

pub async fn generate(path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let chunks = collect_code_chunks(&path);
    let readme = generate_readme(&chunks).await?;

    let out_path = path.join("README.md");
    write(out_path, readme)?;
    println!("✅ README.md generated");
    Ok(())
}
