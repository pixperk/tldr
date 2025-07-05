use crate::llm::client::LlmClient;
use crate::llm::prompt::SYSTEM_PROMPT;
use serde_json::json;
use std::sync::Arc;
use tokio::time::Duration;
use indicatif::{ProgressBar, ProgressStyle};

pub struct SpeedOptimizedGenerator {
    client: Arc<LlmClient>,
}

impl SpeedOptimizedGenerator {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Arc::new(LlmClient::new(api_key)),
        }
    }

    pub async fn generate_readme_fast(
        &self,
        chunks: &[String],
    ) -> Result<String, Box<dyn std::error::Error>> {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏ ")
                .template("{spinner:.cyan} {msg}")
                .unwrap()
        );
        pb.enable_steady_tick(Duration::from_millis(100));
        pb.set_message("🚀 Generating optimized README...");

        let filtered_chunks = self.filter_important_chunks(chunks);
        let context = self.build_smart_context(&filtered_chunks);
        
        pb.set_message("📝 Calling Gemini API...");
        let readme_content = self.call_api(&context).await?;
        
        pb.finish_with_message("✅ README generated successfully!");
        Ok(readme_content)
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
               file_path.contains("cli") ||
               file_path.contains("lib.rs") ||
               file_path.contains("mod.rs") {
                important.push(chunk.clone());
            } else if !file_path.contains("test") && 
                     !file_path.contains("target") &&
                     chunk.len() < 10000 {
                other.push(chunk.clone());
            }
        }

        important.extend(other.into_iter().take(8));
        important
    }

    fn build_smart_context(&self, chunks: &[String]) -> String {
        let combined_content = chunks.join("\n\n");
        
        if combined_content.len() > 50000 {
            let truncated = &combined_content[..50000];
            format!("{}\n\n... [Content truncated for API efficiency] ...", truncated)
        } else {
            combined_content
        }
    }

    async fn call_api(&self, context: &str) -> Result<String, Box<dyn std::error::Error>> {
        let prompt = format!(
            "{}\n\nAnalyze this codebase and generate a comprehensive README.md:\n\n{}",
            SYSTEM_PROMPT,
            context
        );

        let response = self.client.client()
            .post("https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent")
            .query(&[("key", self.client.api_key())])
            .header("Content-Type", "application/json")
            .json(&json!({
                "contents": [{
                    "parts": [{
                        "text": prompt
                    }]
                }]
            }))
            .send()
            .await?;

        let json: serde_json::Value = response.json().await?;

        if let Some(error) = json.get("error") {
            return Err(format!("Gemini API Error: {}", error).into());
        }

        let text = json["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .ok_or("Failed to parse API response")?
            .to_string();

        Ok(text)
    }
}
