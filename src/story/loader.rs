use crate::engine::{room::Room, item::Item};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Story {
    pub story: StoryInfo,
    pub rooms: HashMap<String, Room>,
    pub items: HashMap<String, Item>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoryInfo {
    pub title: String,
    pub start_room: String,
}

pub fn load_story(path: &Path) -> Result<Story, Box<dyn std::error::Error>> {
    let mut file_path = path.to_path_buf();
    
    // If path is a directory, look for the .toml file inside it
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let file_path_entry = entry.path();
            if file_path_entry.extension().map_or(false, |ext| ext == "toml") {
                file_path = file_path_entry;
                break;
            }
        }
        
        // Check if we found a TOML file
        if file_path == path.to_path_buf() {
            return Err("No .toml file found in directory".into());
        }
    }
    
    let content = fs::read_to_string(&file_path)?;
    let story: Story = toml::from_str(&content)?;
    
    // Validate the story has a start room
    if !story.rooms.contains_key(&story.story.start_room) {
        return Err("Start room does not exist in story".into());
    }
    
    Ok(story)
}// TOML parser working :0

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_load_valid_story() {
        let test_content = r#"
[story]
title = "Test Story"
start_room = "start"

[rooms.start]
title = "Start Room"
description = "A starting room."
items = ["key"]

[items.key]
name = "Test Key"
description = "A test key."
"#;

        // Write to a temporary file
        let test_path = "test_load_valid_story.toml";
        fs::write(test_path, test_content).expect("Failed to write test file");

        let result = load_story(Path::new(test_path));
        assert!(result.is_ok());

        let story = result.unwrap();
        assert_eq!(story.story.title, "Test Story");
        assert_eq!(story.story.start_room, "start");
        assert!(story.rooms.contains_key("start"));
        assert!(story.items.contains_key("key"));

        // Clean up
        fs::remove_file(test_path).expect("Failed to remove test file");
    }

    #[test]
    fn test_load_invalid_story_missing_start_room() {
        let test_content = r#"
[story]
title = "Test Story"
start_room = "nonexistent"

[rooms.start]
title = "Start Room"
description = "A starting room."
items = ["key"]

[items.key]
name = "Test Key"
description = "A test key."
"#;

        // Write to a temporary file
        let test_path = "test_load_invalid_story.toml";
        fs::write(test_path, test_content).expect("Failed to write test file");

        let result = load_story(Path::new(test_path));
        assert!(result.is_err());

        // Clean up
        fs::remove_file(test_path).expect("Failed to remove test file");
    }

    #[test]
    fn test_load_invalid_toml() {
        let test_content = r#"
[story]
title = "Test Story"
start_room = "start"

[rooms.start
title = "Start Room"
"#;

        // Write to a temporary file
        let test_path = "test_load_invalid_toml.toml";
        fs::write(test_path, test_content).expect("Failed to write test file");

        let result = load_story(Path::new(test_path));
        assert!(result.is_err());

        // Clean up
        fs::remove_file(test_path).expect("Failed to remove test file");
    }
}
