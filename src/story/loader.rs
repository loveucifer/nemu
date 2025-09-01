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
    let content = fs::read_to_string(path)?;
    let story: Story = toml::from_str(&content)?;
    
    // Validate the story has a start room
    if !story.rooms.contains_key(&story.story.start_room) {
        return Err("Start room does not exist in story".into());
    }
    
    Ok(story)
}