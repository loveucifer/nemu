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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_validate_valid_story() {
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
        let test_path = "test_validate_valid_story.toml";
        fs::write(test_path, test_content).expect("Failed to write test file");

        let result = handle_validate(PathBuf::from(test_path)).await;
        assert!(result.is_ok());

        // Clean up
        fs::remove_file(test_path).expect("Failed to remove test file");
    }

    #[tokio::test]
    async fn test_validate_invalid_story() {
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
        let test_path = "test_validate_invalid_story.toml";
        fs::write(test_path, test_content).expect("Failed to write test file");

        let result = handle_validate(PathBuf::from(test_path)).await;
        assert!(result.is_err());

        // Clean up
        fs::remove_file(test_path).expect("Failed to remove test file");
    }
}