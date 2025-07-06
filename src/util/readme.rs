use crate::llm::LlmGenerator;
use crate::llm::prompt::SYSTEM_PROMPT;
use crate::cli::LlmProvider;
use crate::rag::collect_code_chunks;
use std::path::PathBuf;
use std::fs;
use std::env;

pub async fn generate(
    path: PathBuf, 
    provider: LlmProvider, 
    api_key: Option<String>,
    streaming: bool,
    custom_prompt: Option<String>,
    prompt_file: Option<PathBuf>,
    instructions: Option<String>,
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

    // Build the final prompt
    let final_prompt = build_final_prompt(custom_prompt, prompt_file, instructions)?;
    
    let generator = LlmGenerator::new(provider.clone(), api_key);
    
    let provider_name = match provider {
        LlmProvider::Gemini => "Gemini",
        LlmProvider::OpenAI => "OpenAI",
    };
    
    // Use streaming mode when flag is present, otherwise use fast mode
    if streaming {
        println!("🚀 Generating README with {} (streaming mode)...", provider_name);
        generator.generate_readme_streaming(&chunks, &out_path, &final_prompt).await?;
    } else {
        println!("🚀 Generating README with {} (fast mode)...", provider_name);
        let readme_content = generator.generate_readme_fast(&chunks, &final_prompt).await?;
        fs::write(&out_path, readme_content)?;
        println!("🎉 README.md generated successfully at: {}", out_path.display());
    }
    
    Ok(())
}

fn build_final_prompt(
    custom_prompt: Option<String>,
    prompt_file: Option<PathBuf>,
    instructions: Option<String>,
) -> Result<String, Box<dyn std::error::Error>> {
    // Start with the base prompt - either custom, from file, or default
    let mut final_prompt = match (custom_prompt, prompt_file) {
        (Some(prompt), None) => {
            println!("📝 Using custom prompt from CLI");
            prompt
        }
        (None, Some(file_path)) => {
            println!("📂 Loading prompt from file: {}", file_path.display());
            fs::read_to_string(file_path)
                .map_err(|e| format!("Failed to read prompt file: {}", e))?
        }
        (Some(_), Some(_)) => {
            return Err("Cannot specify both --prompt and --prompt-file options".into());
        }
        (None, None) => {
            println!("📋 Using default system prompt");
            SYSTEM_PROMPT.to_string()
        }
    };

    // Append extra instructions if provided
    if let Some(extra_instructions) = instructions {
        println!("📝 Appending extra instructions");
        final_prompt.push_str("\n\n**Additional Instructions:**\n");
        final_prompt.push_str(&extra_instructions);
    }

    Ok(final_prompt)
}