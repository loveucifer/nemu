use std::fs;
use std::path::PathBuf;

pub async fn handle_create(name: String) -> Result<(), Box<dyn std::error::Error>> {
    let dir_path = PathBuf::from(&name);
    fs::create_dir_all(&dir_path)?;
    
    let toml_content = format!(r#"[story]
title = "{}"
start_room = "entrance"

[rooms.entrance]
title = "Front Door"
description = "A wooden door stands before you. [NORTH] leads to the living room."
north = "living_room"
items = ["key"]

[rooms.living_room]
title = "Living Room"
description = "A cozy room with a fireplace. [SOUTH] leads back to the entrance."
south = "entrance"
items = ["book"]

[items.key]
name = "brass key"
description = "An old brass key."

[items.book]
name = "leather-bound book"
description = "A book with strange symbols on its cover."
"#, name);
    
    let file_path = dir_path.join(format!("{}.toml", name.replace(" ", "_")));
    fs::write(file_path, toml_content)?;
    
    println!("Created new story '{}'! :D", name);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[tokio::test]
    async fn test_create_story() {
        let story_name = "test_new_story";
        let dir_path = Path::new(story_name);
        
        // Clean up if directory exists
        if dir_path.exists() {
            fs::remove_dir_all(dir_path).ok();
        }
        
        let result = handle_create(story_name.to_string()).await;
        assert!(result.is_ok());
        
        // Check if directory was created
        assert!(dir_path.exists());
        
        // Check if TOML file was created
        let toml_path = dir_path.join("test_new_story.toml");
        assert!(toml_path.exists());
        
        // Read and verify content
        let content = fs::read_to_string(toml_path).expect("Failed to read created file");
        assert!(content.contains("title = \"test_new_story\""));
        assert!(content.contains("[rooms.entrance]"));
        assert!(content.contains("[items.key]"));
        
        // Clean up
        fs::remove_dir_all(dir_path).expect("Failed to clean up test directory");
    }
}
