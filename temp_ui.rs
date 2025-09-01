use crate::core::{Room, GameError};
use crossterm::{
    execute,
    terminal::{Clear, ClearType},
    cursor::MoveTo,
    style::{Print, SetForegroundColor, ResetColor, Color},
};
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

pub mod input_handler;

pub struct TerminalUI {
    input_handler: input_handler::InputHandler,
}

impl TerminalUI {
    pub fn new() -> Result<Self, GameError> {
        Ok(Self {
            input_handler: input_handler::InputHandler::new(),
        })
    }
    
    pub fn clear_screen(&self) -> Result<(), GameError> {
        execute!(io::stdout(), Clear(ClearType::All))?;
        Ok(())
    }
    
    pub fn print_welcome(&self, title: &str) -> Result<(), GameError> {
        // Print title with color
        execute!(
            io::stdout(),
            SetForegroundColor(Color::Cyan),
            Print("=========================================\n"),
            ResetColor
        )?;
        
        execute!(
            io::stdout(),
            SetForegroundColor(Color::Yellow),
            Print(format!("    Welcome to: {}\n", title)),
            ResetColor
        )?;
        
        execute!(
            io::stdout(),
            SetForegroundColor(Color::Cyan),
            Print("=========================================\n\n"),
            ResetColor
        )?;
        
        Ok(())
    }
    
    pub fn print_room_description(&self, room: &Room) -> Result<(), GameError> {
        // Print room title with color based on atmosphere
        let title_color = match room.atmosphere.as_str() {
            "mysterious" | "eerie" => Color::Blue,
            "awe-inspiring" | "transcendent" => Color::Magenta,
            "warm" | "friendly" => Color::Green,
            _ => Color::White,
        };
        
        execute!(
            io::stdout(),
            SetForegroundColor(title_color),
            Print(format!("{}\n", room.title)),
            ResetColor
        )?;
        
        // Print description with appropriate color
        let desc_color = match room.lighting.as_str() {
            "dim_blue" => Color::Blue,
            "bright" => Color::Yellow,
            "multicolor" => Color::Rgb { r: 100, g: 200, b: 255 },
            _ => Color::White,
        };
        
        execute!(
            io::stdout(),
            SetForegroundColor(desc_color),
            Print(format!("{}\n", room.description)),
            ResetColor
        )?;
        
        println!();
        Ok(())
    }
    
    pub fn print_message(&self, message: &str) -> Result<(), GameError> {
        // Add typing effect animation for narrative messages
        if message.len() < 200 { // Only animate shorter messages
            for c in message.chars() {
                print!("{}", c);
                io::stdout().flush()?;
                // Add slight delay for typing effect
                if ![ , '\n', ., !, ?].contains(&c) {
                    thread::sleep(Duration::from_millis(15));
                }
            }
        } else {
            print!("{}", message);
        }
        
        println!();
        println!();
        Ok(())
    }
    
    pub fn print_goodbye(&self) -> Result<(), GameError> {
        println!();
        execute!(
            io::stdout(),
            SetForegroundColor(Color::Green),
            Print("Thanks for playing! :D\n"),
            ResetColor
        )?;
        Ok(())
    }
    
    pub fn get_user_input(&mut self) -> Result<String, GameError> {
        execute!(
            io::stdout(),
            SetForegroundColor(Color::Cyan),
            Print("> "),
            ResetColor
        )?;
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        let input = input.trim().to_string();
        if !input.is_empty() {
            self.input_handler.add_to_history(&input);
        }
        
        Ok(input)
    }
    
    pub fn print_combat_message(&self, message: &str) -> Result<(), GameError> {
        execute!(
            io::stdout(),
            SetForegroundColor(Color::Red),
            Print(format!("COMBAT: {}\n", message)),
            ResetColor
        )?;
        Ok(())
    }
    
    pub fn print_status(&self, health: u32, max_health: u32, experience: u32) -> Result<(), GameError> {
        let health_percent = (health as f32 / max_health as f32) * 100.0;
        let health_bar = format!("[{:<20}] {:.1}% HP", 
            "█".repeat((health_percent / 5.0) as usize), 
            health_percent);
        
        execute!(
            io::stdout(),
            SetForegroundColor(Color::Green),
            Print(format!("Health: {}\n", health_bar)),
            ResetColor
        )?;
        
        execute!(
            io::stdout(),
            SetForegroundColor(Color::Yellow),
            Print(format!("Experience: {}\n", experience)),
            ResetColor
        )?;
        
        Ok(())
    }
    
    pub fn update_context(&mut self, items: Vec<String>, npcs: Vec<String>, exits: Vec<String>) {
        self.input_handler.update_context(items, npcs, exits);
    }
}
