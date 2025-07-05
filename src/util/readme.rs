use crate::llm::SpeedOptimizedGenerator;
use crate::rag::collect_code_chunks;
use std::path::PathBuf;
use std::fs;
use std::env;

pub async fn generate(path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let chunks = collect_code_chunks(&path);
    let out_path = path.join("README.md");
    
    let api_key = env::var("GEMINI_API_KEY")?;
    let generator = SpeedOptimizedGenerator::new(api_key);
    
    println!("🚀 Speed-optimized README generation for large codebase...");
    let readme_content = generator.generate_readme_fast(&chunks).await?;
    
    fs::write(&out_path, readme_content)?;
    
    println!("🎉 README.md generated successfully at: {}", out_path.display());
    
    Ok(())
}