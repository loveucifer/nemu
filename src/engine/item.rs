use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Item {
    pub name: String,
    pub description: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_creation() {
        let item = Item {
            name: "Test Item".to_string(),
            description: "A test item".to_string(),
        };

        assert_eq!(item.name, "Test Item");
        assert_eq!(item.description, "A test item");
    }
}