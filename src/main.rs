use clap::Parser;
use std::path::PathBuf;

mod cli;
mod engine;
mod story;

#[derive(Parser)]
#[command(name = "nemu", about = "Nemu - Text Adventure Engine")]
enum NemuCommand {
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
    let command = NemuCommand::parse();
    
    match command {
        NemuCommand::Play { file } => {
            cli::play::handle_play(file).await?;
        },
        NemuCommand::Create { name } => {
            cli::create::handle_create(name).await?;
        },
        NemuCommand::Validate { file } => {
            cli::validate::handle_validate(file).await?;
        },
    }
    
    Ok(())
}// CLI commands working nicely :0
// v1.0 ready to ship :D
