use crate::llm::client::LlmClient;
use crate::llm::streaming::{LlmApiClient, StreamingSectionGenerator};
use serde_json::json;
use std::sync::Arc;
use tokio::time::Duration;
use indicatif::{ProgressBar, ProgressStyle};
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct OpenAiGenerator {
    client: Arc<LlmClient>,
}

impl LlmApiClient for OpenAiGenerator {
    async fn call_with_custom_context(&self, section_prompt: &str, custom_prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
        let combined_prompt = format!(
            "Context: {}\n\nTask: {}",
            custom_prompt,
            section_prompt
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
                        "content": "You are a technical writing assistant. Follow the provided context and guidelines while completing the specific task."
                    },
                    {
                        "role": "user",
                        "content": combined_prompt
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

    async fn call_main(&self, context: &str, custom_prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
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

        let filtered_chunks = StreamingSectionGenerator::filter_important_chunks(chunks);
        let context = StreamingSectionGenerator::build_smart_context(&filtered_chunks);
        
        pb.set_message("📝 Calling OpenAI API...");
        let readme_content = self.call_main(&context, custom_prompt).await?;
        
        pb.finish_with_message("✅ README generated successfully!");
        Ok(readme_content)
    }

    pub async fn generate_readme_streaming(
        &self,
        chunks: &[String],
        output_path: &std::path::Path,
        custom_prompt: &str,
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

        let filtered_chunks = StreamingSectionGenerator::filter_important_chunks(chunks);
        let generator = StreamingSectionGenerator::new();
        generator.generate_sections_incrementally(self, &filtered_chunks, &mut file, &pb, custom_prompt).await?;
        
        // Write final section combination
        pb.set_message("🔗 Finalizing README structure...");
        generator.write_final_sections(&mut file).await?;
        
        pb.finish_with_message("✅ README.md generated with OpenAI streaming writes!");
        println!("📝 README.md written to: {}", output_path.display());
        
        Ok(())
    }
}
