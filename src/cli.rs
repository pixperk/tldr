use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "tldrs", about = "Generate docs from codebase using LLMs")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate README.md
    Readme {
        /// Path to project dir
        #[arg(default_value = ".")]
        path: PathBuf,
        
        /// LLM provider to use
        #[arg(short, long, default_value = "gemini")]
        provider: LlmProvider,
        
        /// API key for the selected provider
        #[arg(short, long)]
        api_key: Option<String>,
        
        /// Use streaming mode to write README sections as they are generated
        #[arg(short, long)]
        streaming: bool,
        
        /// Custom prompt for README generation (overrides default system prompt)
        #[arg(long)]
        prompt: Option<String>,
        
        /// Path to a file containing custom prompt (overrides default system prompt)
        #[arg(long)]
        prompt_file: Option<PathBuf>,
        
        /// Additional instructions to append to the prompt (works with default or custom prompts)
        #[arg(long)]
        instructions: Option<String>,
    }
}

#[derive(Clone, ValueEnum)]
pub enum LlmProvider {
    /// Google Gemini
    Gemini,
    /// OpenAI GPT
    OpenAI,
}