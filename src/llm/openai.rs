use crate::llm::client::LlmClient;
use crate::llm::prompt::SYSTEM_PROMPT;
use serde_json::json;
use std::sync::Arc;
use tokio::time::Duration;
use indicatif::{ProgressBar, ProgressStyle};

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
        let readme_content = self.call_openai_api(&context).await?;
        
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

    async fn call_openai_api(&self, context: &str) -> Result<String, Box<dyn std::error::Error>> {
        let prompt = format!(
            "Analyze this codebase and generate a comprehensive README.md:\n\n{}",
            context
        );

        let response = self.client.client()
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.client.api_key()))
            .header("Content-Type", "application/json")
            .json(&json!({
                "model": "gpt-4",
                "messages": [
                    {
                        "role": "system",
                        "content": SYSTEM_PROMPT
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
}
