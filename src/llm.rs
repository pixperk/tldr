use reqwest::Client;
use serde_json::json;
use std::env;

pub async fn generate_readme(chunks: &[String]) -> Result<String, Box<dyn std::error::Error>> {
    let api_key = env::var("GEMINI_API_KEY")?;
    let client = Client::new();

    let context = chunks.join("\n\n");

    let prompt = format!(
        "You are a helpful Rust documentation bot. Generate a beautiful, production-grade README.md for the following codebase:\n\n{}",
        context
    );

    let response = client
        .post("https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent")
        .query(&[("key", &api_key)])
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

    // Check if the API returned an error
    if let Some(error) = json.get("error") {
        return Err(format!("Gemini API Error: {}", error).into());
    }

    let readme_text = json["candidates"][0]["content"]["parts"][0]["text"]
        .as_str()
        .ok_or_else(|| format!("Failed to parse README text from response. Full response: {}", serde_json::to_string_pretty(&json).unwrap_or_else(|_| "Unable to serialize response".to_string())))?
        .to_string();
    
    Ok(readme_text)
}
