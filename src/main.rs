mod rag;
mod util;
mod llm;
mod cli;


use clap::Parser;
use dotenvy::dotenv;
use cli::{Cli, Commands};

use crate::util::readme;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let cli = Cli::parse();

    match cli.command {
        Commands::Readme { path, provider, api_key } => {
            if let Err(err) = readme::generate(path, provider, api_key).await{
                eprintln!("❌ Error generating README: {}", err);
            };
        }
    }
}
