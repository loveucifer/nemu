use crate::{engine::game::Game, story::loader::load_story};
use std::path::PathBuf;

pub async fn handle_play(file: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting game: {:?}", file);
    
    let story = load_story(&file)?;
    let mut game = Game::new(story)?;
    
    game.run().await?;
    
    Ok(())
}