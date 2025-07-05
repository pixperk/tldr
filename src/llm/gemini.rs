use crate::llm::client::LlmClient;
use crate::llm::prompt::SYSTEM_PROMPT;
use serde_json::json;
use std::env;
use indicatif::{ProgressBar, ProgressStyle};

pub async fn generate_readme(chunks: &[String]) -> Result<String, Box<dyn std::error::Error>> {
    let api_key = env::var("GEMINI_API_KEY")?;
    let client = LlmClient::new(api_key);
    
    let context = chunks.join("\n\n");
    let prompt = format!("{}\n\n{}", SYSTEM_PROMPT, context);

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏ ")
            .template("{spinner:.cyan} {msg}")
            .unwrap()
    );
    pb.enable_steady_tick(std::time::Duration::from_millis(100));
    pb.set_message("Generating README with Gemini AI...");

    let response = client.client()
        .post("https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent")
        .query(&[("key", client.api_key())])
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

    pb.set_message("Processing response...");

    let json: serde_json::Value = response.json().await?;

    if let Some(error) = json.get("error") {
        pb.finish_with_message("❌ API Error");
        return Err(format!("Gemini API Error: {}", error).into());
    }

    let readme_text = json["candidates"][0]["content"]["parts"][0]["text"]
        .as_str()
        .ok_or_else(|| format!("Failed to parse README text from response"))?
        .to_string();

    
    Ok(readme_text)
}
