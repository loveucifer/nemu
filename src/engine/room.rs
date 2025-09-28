use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Room {
    pub title: String,
    pub description: String,
    #[serde(flatten)]
    pub exits: HashMap<String, String>, // direction -> room_id
    pub items: Vec<String>, // item IDs
}// Add basic data structures :D

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_room_creation() {
        let mut exits = HashMap::new();
        exits.insert("north".to_string(), "other_room".to_string());
        
        let room = Room {
            title: "Test Room".to_string(),
            description: "A test room".to_string(),
            exits,
            items: vec!["key".to_string()],
        };

        assert_eq!(room.title, "Test Room");
        assert_eq!(room.description, "A test room");
        assert!(room.exits.contains_key("north"));
        assert_eq!(room.exits.get("north").unwrap(), "other_room");
        assert!(room.items.contains(&"key".to_string()));
    }
}
