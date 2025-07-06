use crate::cli::LlmProvider;
use crate::llm::{SpeedOptimizedGenerator, OpenAiGenerator};
use std::path::Path;

#[derive(Debug, Clone)]
pub enum LlmGenerator {
    Gemini(SpeedOptimizedGenerator),
    OpenAI(OpenAiGenerator),
}

impl LlmGenerator {
    pub fn new(provider: LlmProvider, api_key: String) -> Self {
        match provider {
            LlmProvider::Gemini => Self::Gemini(SpeedOptimizedGenerator::new(api_key)),
            LlmProvider::OpenAI => Self::OpenAI(OpenAiGenerator::new(api_key)),
        }
    }

    pub async fn generate_readme_fast(
        &self,
        chunks: &[String],
        custom_prompt: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        match self {
            Self::Gemini(generator) => generator.generate_readme_fast(chunks, custom_prompt).await,
            Self::OpenAI(generator) => generator.generate_readme_fast(chunks, custom_prompt).await,
        }
    }

    pub async fn generate_readme_streaming(
        &self,
        chunks: &[String],
        output_path: &Path,
        custom_prompt: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Self::Gemini(generator) => generator.generate_readme_streaming(chunks, output_path, custom_prompt).await,
            Self::OpenAI(generator) => generator.generate_readme_streaming(chunks, output_path, custom_prompt).await,
        }
    }
}
