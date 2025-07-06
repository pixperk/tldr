use crate::llm::client::LlmClient;
use serde_json::json;
use std::sync::Arc;
use tokio::time::Duration;
use indicatif::{ProgressBar, ProgressStyle};
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct OpenAiGenerator {
    client: Arc<LlmClient>,
}

impl OpenAiGenerator {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Arc::new(LlmClient::new(api_key)),
        }
    }

    pub async fn generate_readme_fast(
        &self,
        chunks: &[String],
        custom_prompt: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏ ")
                .template("{spinner:.cyan} {msg}")
                .unwrap()
        );
        pb.enable_steady_tick(Duration::from_millis(100));
        pb.set_message("🤖 Generating README with OpenAI...");

        let filtered_chunks = self.filter_important_chunks(chunks);
        let context = self.build_smart_context(&filtered_chunks);
        
        pb.set_message("📝 Calling OpenAI API...");
        let readme_content = self.call_openai_api(&context, custom_prompt).await?;
        
        pb.finish_with_message("✅ README generated successfully!");
        Ok(readme_content)
    }

    pub async fn generate_readme_streaming(
        &self,
        chunks: &[String],
        output_path: &std::path::Path,
        _custom_prompt: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        use std::io::Write;
        
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏ ")
                .template("{spinner:.cyan} {msg}")
                .unwrap()
        );
        pb.enable_steady_tick(Duration::from_millis(100));
        pb.set_message("🤖 Generating README with OpenAI streaming writes...");

        let mut file = std::fs::File::create(output_path)?;
        
        // Write minimal initial header
        let header = "<!-- README being generated with OpenAI... -->\n\n";
        file.write_all(header.as_bytes())?;
        file.flush()?;
        pb.set_message("📄 README file created, generating sections...");

        let filtered_chunks = self.filter_important_chunks(chunks);
        self.generate_sections_incrementally(&filtered_chunks, &mut file, &pb).await?;
        
        // Write final section combination
        pb.set_message("🔗 Finalizing README structure...");
        self.combine_sections_and_write(&mut file).await?;
        
        pb.finish_with_message("✅ README.md generated with OpenAI streaming writes!");
        println!("📝 README.md written to: {}", output_path.display());
        
        Ok(())
    }

    fn filter_important_chunks(&self, chunks: &[String]) -> Vec<String> {
        let mut important = Vec::new();
        let mut other = Vec::new();

        for chunk in chunks {
            let lines: Vec<&str> = chunk.lines().collect();
            if lines.is_empty() {
                continue;
            }
            
            let file_path = lines[0].trim_end_matches(':').to_lowercase();
            
            if file_path.contains("main") || 
               file_path.contains("cargo.toml") || 
               file_path.contains("package.json") ||
               file_path.contains("cli") ||
               file_path.contains("lib.rs") ||
               file_path.contains("mod.rs") {
                important.push(chunk.clone());
            } else if !file_path.contains("test") && 
                     !file_path.contains("target") &&
                     !file_path.contains("node_modules") &&
                     chunk.len() < 10000 {
                other.push(chunk.clone());
            }
        }

        important.extend(other.into_iter().take(8));
        important
    }

    fn build_smart_context(&self, chunks: &[String]) -> String {
        let combined_content = chunks.join("\n\n");
        
        if combined_content.len() > 60000 {
            let truncated = &combined_content[..60000];
            format!("{}\n\n... [Content truncated for API efficiency] ...", truncated)
        } else {
            combined_content
        }
    }

    // Simple API call for streaming sections that still use the default system prompt
    async fn call_openai_api_simple(&self, context: &str) -> Result<String, Box<dyn std::error::Error>> {
        let default_system_prompt = "You are a technical writing assistant. Generate high-quality, professional documentation based on the provided code context.";
        
        let response = self.client.client()
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.client.api_key()))
            .header("Content-Type", "application/json")
            .json(&json!({
                "model": "gpt-4o",
                "messages": [
                    {
                        "role": "system",
                        "content": default_system_prompt
                    },
                    {
                        "role": "user",
                        "content": context
                    }
                ],
                "max_tokens": 1000,
                "temperature": 0.7
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(format!("OpenAI API error: {}", error_text).into());
        }

        let response_json: serde_json::Value = response.json().await?;
        
        if let Some(content) = response_json["choices"][0]["message"]["content"].as_str() {
            Ok(content.trim().to_string())
        } else {
            Err("Invalid response format from OpenAI API".into())
        }
    }

    async fn call_openai_api(&self, context: &str, custom_prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
        let prompt = format!(
            "Analyze this codebase and generate a comprehensive README.md:\n\n{}",
            context
        );

        let response = self.client.client()
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.client.api_key()))
            .header("Content-Type", "application/json")
            .json(&json!({
                "model": "gpt-4o",
                "messages": [
                    {
                        "role": "system",
                        "content": custom_prompt
                    },
                    {
                        "role": "user",
                        "content": prompt
                    }
                ],
                "max_tokens": 4000,
                "temperature": 0.7
            }))
            .send()
            .await?;

        let json: serde_json::Value = response.json().await?;

        if let Some(error) = json.get("error") {
            return Err(format!("OpenAI API Error: {}", error).into());
        }

        let text = json["choices"][0]["message"]["content"]
            .as_str()
            .ok_or("Failed to parse OpenAI response")?
            .to_string();

        Ok(text)
    }

    async fn generate_sections_incrementally(
        &self,
        chunks: &[String],
        file: &mut std::fs::File,
        pb: &ProgressBar,
    ) -> Result<(), Box<dyn std::error::Error>> {
        use std::io::Write;
        
        let context = self.build_smart_context(chunks);
        
        // Write title line by line
        pb.set_message("📝 Generating title...");
        println!("🔄 Generating project title...");
        let title_prompt = format!(
            "Generate ONLY a project title in markdown format (# Title). Be concise:\n\n{}",
            &context[..context.len().min(3000)]
        );
        let title = self.call_openai_api_simple(&title_prompt).await?;
        for line in title.lines() {
            file.write_all(line.as_bytes())?;
            file.write_all("\n".as_bytes())?;
            file.flush()?;
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
        file.write_all("\n".as_bytes())?;
        file.flush()?;
        println!("✅ Title written");
        
        // Write description sentence by sentence
        pb.set_message("📝 Writing description...");
        println!("🔄 Generating description...");
        let desc_prompt = format!(
            "Generate ONLY a brief project description (2-3 sentences). No headers:\n\n{}",
            &context[..context.len().min(4000)]
        );
        let description = self.call_openai_api_simple(&desc_prompt).await?;
        for sentence in description.split('.') {
            if !sentence.trim().is_empty() {
                file.write_all(sentence.trim().as_bytes())?;
                file.write_all(". ".as_bytes())?;
                file.flush()?;
                tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
            }
        }
        file.write_all("\n\n".as_bytes())?;
        file.flush()?;
        println!("✅ Description written");
        
        // Features section - header first, then content
        pb.set_message("✨ Adding features...");
        println!("🔄 Generating features...");
        file.write_all("## ✨ Features\n\n".as_bytes())?;
        file.flush()?;
        tokio::time::sleep(tokio::time::Duration::from_millis(150)).await;
        
        let features_prompt = format!(
            "Generate ONLY a bulleted list of key features. No headers:\n\n{}",
            &context[..context.len().min(6000)]
        );
        let features = self.call_openai_api_simple(&features_prompt).await?;
        for line in features.lines() {
            if !line.trim().is_empty() {
                file.write_all(line.as_bytes())?;
                file.write_all("\n".as_bytes())?;
                file.flush()?;
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
        }
        file.write_all("\n".as_bytes())?;
        file.flush()?;
        println!("✅ Features written");
        
        // Installation section
        pb.set_message("🔧 Adding installation...");
        println!("🔄 Generating installation guide...");
        file.write_all("## 🚀 Installation\n\n".as_bytes())?;
        file.flush()?;
        tokio::time::sleep(tokio::time::Duration::from_millis(150)).await;
        
        let install_prompt = format!(
            "Generate ONLY installation instructions. No headers:\n\n{}",
            &context[..context.len().min(4000)]
        );
        let installation = self.call_openai_api_simple(&install_prompt).await?;
        for line in installation.lines() {
            if !line.trim().is_empty() {
                file.write_all(line.as_bytes())?;
                file.write_all("\n".as_bytes())?;
                file.flush()?;
                tokio::time::sleep(tokio::time::Duration::from_millis(80)).await;
            }
        }
        file.write_all("\n".as_bytes())?;
        file.flush()?;
        println!("✅ Installation written");
        
        // Usage section
        pb.set_message("📖 Adding usage...");
        println!("🔄 Generating usage examples...");
        file.write_all("## 📚 Usage\n\n".as_bytes())?;
        file.flush()?;
        tokio::time::sleep(tokio::time::Duration::from_millis(150)).await;
        
        let usage_prompt = format!(
            "Generate ONLY usage examples and basic commands. No headers:\n\n{}",
            &context[..context.len().min(5000)]
        );
        let usage = self.call_openai_api_simple(&usage_prompt).await?;
        for line in usage.lines() {
            if !line.trim().is_empty() {
                file.write_all(line.as_bytes())?;
                file.write_all("\n".as_bytes())?;
                file.flush()?;
                tokio::time::sleep(tokio::time::Duration::from_millis(80)).await;
            }
        }
        file.write_all("\n".as_bytes())?;
        file.flush()?;
        println!("✅ Usage written");
        
        Ok(())
    }

    async fn combine_sections_and_write(
        &self,
        file: &mut std::fs::File,
    ) -> Result<(), Box<dyn std::error::Error>> {
        use std::io::Write;
        
        println!("🔄 Adding final sections...");
        
        // Write Contributing section
        file.write_all("## 🤝 Contributing\n\n".as_bytes())?;
        file.flush()?;
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        
        file.write_all("Contributions are welcome! ".as_bytes())?;
        file.flush()?;
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        
        file.write_all("Please feel free to submit a Pull Request.\n\n".as_bytes())?;
        file.flush()?;
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        
        // Write License section
        file.write_all("## 📄 License\n\n".as_bytes())?;
        file.flush()?;
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        
        file.write_all("This project is licensed under the MIT License - ".as_bytes())?;
        file.flush()?;
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        
        file.write_all("see the LICENSE file for details.\n".as_bytes())?;
        file.flush()?;
        
        println!("✅ Final sections written");
        
        Ok(())
    }
}
