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
exits = {{ north = "living_room" }}
items = ["key"]

[rooms.living_room]
title = "Living Room"
description = "A cozy room with a fireplace. [SOUTH] leads back to the entrance."
exits = {{ south = "entrance" }}
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