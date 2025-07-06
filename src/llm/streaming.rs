use std::fs::File;
use std::io::Write;
use indicatif::ProgressBar;
use anyhow::Result;

/// Common interface for LLM API calls with custom context
pub trait LlmApiClient {
    /// Make an API call with custom prompt context for streaming sections
    async fn call_with_custom_context(&self, section_prompt: &str, custom_prompt: &str) -> Result<String, Box<dyn std::error::Error>>;
    
    /// Make a main API call for full README generation
    async fn call_main(&self, context: &str, custom_prompt: &str) -> Result<String, Box<dyn std::error::Error>>;
}

/// Common streaming section generator
pub struct StreamingSectionGenerator;

impl StreamingSectionGenerator {
    pub fn new() -> Self {
        Self
    }

    /// Generate all sections incrementally using the provided API client
    pub async fn generate_sections_incrementally<T: LlmApiClient>(
        &self,
        api_client: &T,
        chunks: &[String],
        file: &mut File,
        pb: &ProgressBar,
        custom_prompt: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let context = Self::build_smart_context(chunks);
        
        // Generate each section
        self.write_title_section(api_client, &context, file, pb, custom_prompt).await?;
        self.write_description_section(api_client, &context, file, pb, custom_prompt).await?;
        self.write_features_section(api_client, &context, file, pb, custom_prompt).await?;
        self.write_installation_section(api_client, &context, file, pb, custom_prompt).await?;
        self.write_usage_section(api_client, &context, file, pb, custom_prompt).await?;
        
        Ok(())
    }

    async fn write_title_section<T: LlmApiClient>(
        &self,
        api_client: &T,
        context: &str,
        file: &mut File,
        pb: &ProgressBar,
        custom_prompt: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        pb.set_message("📝 Generating title...");
        println!("🔄 Generating project title...");
        
        let title_prompt = format!(
            "Generate ONLY a project title in markdown format (# Title). Be concise:\n\n{}",
            &context[..context.len().min(3000)]
        );
        
        let title = api_client.call_with_custom_context(&title_prompt, custom_prompt).await?;
        
        for line in title.lines() {
            file.write_all(line.as_bytes())?;
            file.write_all("\n".as_bytes())?;
            file.flush()?;
        }
        
        file.write_all("\n".as_bytes())?;
        file.flush()?;
        println!("✅ Title written");
        
        Ok(())
    }

    async fn write_description_section<T: LlmApiClient>(
        &self,
        api_client: &T,
        context: &str,
        file: &mut File,
        pb: &ProgressBar,
        custom_prompt: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        pb.set_message("📝 Writing description...");
        println!("🔄 Generating description...");
        
        let desc_prompt = format!(
            "Generate ONLY a brief project description (2-3 sentences). Be compelling:\n\n{}",
            &context[..context.len().min(3000)]
        );
        
        let description = api_client.call_with_custom_context(&desc_prompt, custom_prompt).await?;
        
        for line in description.lines() {
            file.write_all(line.as_bytes())?;
            file.write_all("\n".as_bytes())?;
            file.flush()?;
        }
        
        file.write_all("\n".as_bytes())?;
        file.flush()?;
        println!("✅ Description written");
        
        Ok(())
    }

    async fn write_features_section<T: LlmApiClient>(
        &self,
        api_client: &T,
        context: &str,
        file: &mut File,
        pb: &ProgressBar,
        custom_prompt: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        pb.set_message("✨ Adding features...");
        println!("🔄 Generating features...");
        
        let features_prompt = format!(
            "Generate a ## Features section with bullet points of key capabilities:\n\n{}",
            &context[..context.len().min(4000)]
        );
        
        let features = api_client.call_with_custom_context(&features_prompt, custom_prompt).await?;
        
        for line in features.lines() {
            file.write_all(line.as_bytes())?;
            file.write_all("\n".as_bytes())?;
            file.flush()?;
        }
        
        file.write_all("\n".as_bytes())?;
        file.flush()?;
        println!("✅ Features written");
        
        Ok(())
    }

    async fn write_installation_section<T: LlmApiClient>(
        &self,
        api_client: &T,
        context: &str,
        file: &mut File,
        pb: &ProgressBar,
        custom_prompt: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        pb.set_message("🔧 Adding installation...");
        println!("🔄 Generating installation guide...");
        
        let install_prompt = format!(
            "Generate a ## Installation section with clear setup steps:\n\n{}",
            &context[..context.len().min(4000)]
        );
        
        let installation = api_client.call_with_custom_context(&install_prompt, custom_prompt).await?;
        
        for line in installation.lines() {
            file.write_all(line.as_bytes())?;
            file.write_all("\n".as_bytes())?;
            file.flush()?;
        }
        
        file.write_all("\n".as_bytes())?;
        file.flush()?;
        println!("✅ Installation written");
        
        Ok(())
    }

    async fn write_usage_section<T: LlmApiClient>(
        &self,
        api_client: &T,
        context: &str,
        file: &mut File,
        pb: &ProgressBar,
        custom_prompt: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        pb.set_message("📖 Adding usage...");
        println!("🔄 Generating usage examples...");
        
        let usage_prompt = format!(
            "Generate a ## Usage section with practical examples and command-line usage:\n\n{}",
            &context[..context.len().min(4000)]
        );
        
        let usage = api_client.call_with_custom_context(&usage_prompt, custom_prompt).await?;
        
        for line in usage.lines() {
            file.write_all(line.as_bytes())?;
            file.write_all("\n".as_bytes())?;
            file.flush()?;
        }
        
        file.write_all("\n".as_bytes())?;
        file.flush()?;
        println!("✅ Usage written");
        
        Ok(())
    }

    /// Build smart context from code chunks
    pub fn build_smart_context(chunks: &[String]) -> String {
        let combined = chunks.join("\n\n");
        
        // Limit context size to prevent token overflow
        if combined.len() > 15000 {
            format!("{}...[truncated]", &combined[..15000])
        } else {
            combined
        }
    }

    /// Filter important chunks (common logic)
    pub fn filter_important_chunks(chunks: &[String]) -> Vec<String> {
        let mut filtered = Vec::new();
        
        for chunk in chunks {
            let chunk_lower = chunk.to_lowercase();
            
            // Skip unimportant files
            if chunk_lower.contains("target/") 
                || chunk_lower.contains("node_modules/")
                || chunk_lower.contains(".git/")
                || chunk_lower.contains("build/")
                || chunk_lower.contains("dist/")
                || chunk_lower.contains(".json")
                || chunk_lower.contains(".lock")
                || chunk_lower.contains(".d")
                || chunk_lower.contains("deps/") {
                continue;
            }
            
            // Include important files
            if chunk_lower.contains("cargo.toml")
                || chunk_lower.contains("package.json")
                || chunk_lower.contains("main.")
                || chunk_lower.contains("lib.")
                || chunk_lower.contains("mod.rs")
                || chunk_lower.contains("src/")
                || chunk.lines().count() > 5 {
                filtered.push(chunk.clone());
            }
        }
        
        // Limit to most important chunks
        filtered.into_iter().take(50).collect()
    }

    /// Write final section combination
    pub async fn write_final_sections(
        &self,
        file: &mut File,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let footer = "\n## 🤝 Contributing\n\nContributions are welcome! Please feel free to submit a Pull Request.\n\n## 📄 License\n\nThis project is licensed under the MIT License - see the LICENSE file for details.\n";
        
        file.write_all(footer.as_bytes())?;
        file.flush()?;
        
        Ok(())
    }
}
