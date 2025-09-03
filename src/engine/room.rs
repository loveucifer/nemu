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
