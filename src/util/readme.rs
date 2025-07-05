use crate::llm::LlmGenerator;
use crate::cli::LlmProvider;
use crate::rag::collect_code_chunks;
use std::path::PathBuf;
use std::fs;
use std::env;

pub async fn generate(
    path: PathBuf, 
    provider: LlmProvider, 
    api_key: Option<String>,
    streaming: bool
) -> Result<(), Box<dyn std::error::Error>> {
    let chunks = collect_code_chunks(&path);
    let out_path = path.join("README.md");
    
    let api_key = match api_key {
        Some(key) => key,
        None => {
            let env_var = match provider {
                LlmProvider::Gemini => "GEMINI_API_KEY",
                LlmProvider::OpenAI => "OPENAI_API_KEY",
            };
            env::var(env_var).map_err(|_| {
                format!("API key not provided. Set {} environment variable or use --api-key flag", env_var)
            })?
        }
    };
    
    let generator = LlmGenerator::new(provider.clone(), api_key);
    
    let provider_name = match provider {
        LlmProvider::Gemini => "Gemini",
        LlmProvider::OpenAI => "OpenAI",
    };
    
    // Use streaming mode when flag is present, otherwise use fast mode
    if streaming {
        println!("🚀 Generating README with {} (streaming mode)...", provider_name);
        generator.generate_readme_streaming(&chunks, &out_path).await?;
    } else {
        println!("🚀 Generating README with {} (fast mode)...", provider_name);
        let readme_content = generator.generate_readme_fast(&chunks).await?;
        fs::write(&out_path, readme_content)?;
        println!("🎉 README.md generated successfully at: {}", out_path.display());
    }
    
    Ok(())
}