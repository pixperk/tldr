use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "tldr", about = "Generate docs from codebase using LLMs")]
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
    }
}