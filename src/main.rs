use clap::Parser;
use std::path::PathBuf;

mod cli;
mod engine;
mod story;

#[derive(Parser)]
#[command(name = "bby", about = "BlockyBlocky - Text Adventure Engine")]
enum BbyCommand {
    /// Play a story
    Play {
        file: PathBuf,
    },
    /// Create a new story template
    Create {
        name: String,
    },
    /// Validate a story file
    Validate {
        file: PathBuf,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let command = BbyCommand::parse();
    
    match command {
        BbyCommand::Play { file } => {
            cli::play::handle_play(file).await?;
        },
        BbyCommand::Create { name } => {
            cli::create::handle_create(name).await?;
        },
        BbyCommand::Validate { file } => {
            cli::validate::handle_validate(file).await?;
        },
    }
    
    Ok(())
}