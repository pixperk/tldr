use crate::llm::generate_readme;
use crate::rag::collect_code_chunks;
use std::path::PathBuf;
use std::fs;

pub async fn generate(path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let chunks = collect_code_chunks(&path);
    let out_path = path.join("README.md");
    
    let readme_content = generate_readme(&chunks).await?;
    fs::write(&out_path, readme_content)?;
    
    println!("✅ README.md generated successfully at: {}", out_path.display());
    
    Ok(())
}