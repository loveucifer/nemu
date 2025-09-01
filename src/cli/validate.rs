use crate::story::loader::load_story;
use std::path::PathBuf;

pub async fn handle_validate(file: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    println!("Validating story: {:?}", file);
    
    match load_story(&file) {
        Ok(_) => {
            println!("✓ Story file is valid! :D");
            Ok(())
        },
        Err(e) => {
            eprintln!("✗ Invalid story file: {}", e);
            Err(e)
        }
    }
}