use crate::story::loader::Story;
use crate::engine::{room::Room, parser::{parse_command, Command}};
use crossterm::{
    execute,
    terminal::{Clear, ClearType},
    style::{Print, SetForegroundColor, ResetColor, Color},
};
use std::collections::{HashMap, HashSet};
use std::io::{self, Write};

pub struct Game {
    story: Story,
    current_room: String,
    inventory: HashSet<String>,
}

#[derive(Debug)]
pub enum GameError {
    InvalidRoom,
    ItemNotFound,
    RoomNotFound,
}

impl std::fmt::Display for GameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameError::InvalidRoom => write!(f, "Invalid room"),
            GameError::ItemNotFound => write!(f, "Item not found"),
            GameError::RoomNotFound => write!(f, "Room not found"),
        }
    }
}

impl std::error::Error for GameError {}

impl Game {
    pub fn new(story: Story) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            current_room: story.story.start_room.clone(),
            story,
            inventory: HashSet::new(),
        })
    }
    
    pub async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.clear_screen()?;
        println!("=========================================");
        println!("    Welcome to: {}", self.story.story.title);
        println!("=========================================");
        
        // Show initial room
        self.show_room()?;
        
        loop {
            print!("> ");
            io::stdout().flush()?;
            
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            
            let command = input.trim().to_lowercase();
            
            if command == "quit" || command == "exit" || command == "q" {
                println!("Thanks for playing! :D");
                break;
            }
            
            match self.process_command(&command).await {
                Ok(output) => {
                    if !output.is_empty() {
                        println!("{}", output);
                    }
                }
                Err(e) => println!("Error: {}", e),
            }
            
            // Show room description again after certain commands
            if matches!(parse_command(&command), Command::Go(_) | Command::North | Command::South | Command::East | Command::West) {
                self.show_room()?;
            }
        }
        
        Ok(())
    }
    
    async fn process_command(&mut self, input: &str) -> Result<String, GameError> {
        match parse_command(input) {
            Command::Look => self.look(),
            Command::Go(direction) => self.go(&direction),
            Command::North => self.go("north"),
            Command::South => self.go("south"),
            Command::East => self.go("east"),
            Command::West => self.go("west"),
            Command::Take(item) => self.take(&item),
            Command::Drop(item) => self.drop(&item),
            Command::Inventory => self.show_inventory(),
            Command::Help => Ok(self.help()),
            Command::Unknown => Ok("I don't understand that command. Type 'help' for available commands. :0".to_string()),
        }
    }
    
    fn look(&self) -> Result<String, GameError> {
        let room = self.get_current_room()?;
        let mut output = String::new();
        
        output.push_str(&format!("\n{}", room.title));
        output.push_str(&format!("\n{}", room.description));
        
        if !room.items.is_empty() {
            output.push_str("\n\nYou see: ");
            for (i, item_id) in room.items.iter().enumerate() {
                if let Some(item) = self.story.items.get(item_id) {
                    if i > 0 { output.push_str(", "); }
                    output.push_str(&item.name);
                }
            }
        }
        
        if !room.exits.is_empty() {
            output.push_str("\n\nExits: ");
            let exit_names: Vec<String> = room.exits.keys().cloned().collect();
            output.push_str(&exit_names.join(", "));
        }
        
        Ok(output)
    }
    
    fn go(&mut self, direction: &str) -> Result<String, GameError> {
        let current_room = self.get_current_room()?;
        
        if let Some(next_room_id) = current_room.exits.get(direction) {
            if self.story.rooms.contains_key(next_room_id) {
                self.current_room = next_room_id.clone();
                Ok(format!("You go {}.", direction))
            } else {
                Err(GameError::RoomNotFound)
            }
        } else {
            Ok(format!("You can't go {} from here.", direction))
        }
    }
    
    fn take(&mut self, item_name: &str) -> Result<String, GameError> {
        // Find the item by partial name match
        let mut found_item_id = None;
        {
            let current_room = self.get_current_room()?;
            for item_id in &current_room.items {
                if let Some(item) = self.story.items.get(item_id) {
                    if item.name.to_lowercase().contains(&item_name.to_lowercase()) {
                        found_item_id = Some(item_id.clone());
                        break;
                    }
                }
            }
        }
        
        if let Some(item_id) = found_item_id {
            // We need to remove the item from the room and add to inventory
            if let Some(room) = self.story.rooms.get_mut(&self.current_room) {
                room.items.retain(|id| id != &item_id);
            }
            self.inventory.insert(item_id.clone());
            
            if let Some(item) = self.story.items.get(&item_id) {
                Ok(format!("You take the {}.", item.name))
            } else {
                Ok("You take the item.".to_string())
            }
        } else {
            Err(GameError::ItemNotFound)
        }
    }
    
    fn drop(&mut self, item_name: &str) -> Result<String, GameError> {
        // Find the item in inventory by partial name match
        let mut found_item_id = None;
        for item_id in &self.inventory {
            if let Some(item) = self.story.items.get(item_id) {
                if item.name.to_lowercase().contains(&item_name.to_lowercase()) {
                    found_item_id = Some(item_id.clone());
                    break;
                }
            }
        }
        
        if let Some(item_id) = found_item_id {
            self.inventory.remove(&item_id);
            if let Some(room) = self.story.rooms.get_mut(&self.current_room) {
                room.items.push(item_id.clone());
            }
            
            if let Some(item) = self.story.items.get(&item_id) {
                Ok(format!("You drop the {}.", item.name))
            } else {
                Ok("You drop the item.".to_string())
            }
        } else {
            Err(GameError::ItemNotFound)
        }
    }
    
    fn show_inventory(&self) -> Result<String, GameError> {
        if self.inventory.is_empty() {
            Ok("Your inventory is empty. :0".to_string())
        } else {
            let mut output = "You are carrying:\n".to_string();
            for item_id in &self.inventory {
                if let Some(item) = self.story.items.get(item_id) {
                    output.push_str(&format!("- {}\n", item.name));
                }
            }
            // Remove the trailing newline
            if output.ends_with('\n') {
                output.pop();
            }
            Ok(output)
        }
    }
    
    fn help(&self) -> String {
        "Available commands:\n- look: Look around the current room\n- go [direction]: Move in a direction (north, south, east, west)\n- n/s/e/w: Short forms for directions\n- take [item]: Pick up an item\n- drop [item]: Drop an item\n- inventory: Check your inventory\n- help: Show this help\n- quit: Exit the game\n\nExample: 'go north' or 'take key' :D".to_string()
    }
    
    fn show_room(&self) -> Result<(), Box<dyn std::error::Error>> {
        let room = self.get_current_room()?;
        
        execute!(
            io::stdout(),
            SetForegroundColor(Color::Cyan),
            Print(format!("{}\n", room.title)),
            ResetColor
        )?;
        
        execute!(
            io::stdout(),
            Print(format!("{}\n", room.description)),
        )?;
        
        Ok(())
    }
    
    fn get_current_room(&self) -> Result<&Room, GameError> {
        self.story.rooms.get(&self.current_room).ok_or(GameError::InvalidRoom)
    }
    
    fn clear_screen(&self) -> Result<(), Box<dyn std::error::Error>> {
        execute!(io::stdout(), Clear(ClearType::All))?;
        Ok(())
    }
}// Basic game loop implemented
