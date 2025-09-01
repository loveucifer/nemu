#[derive(Debug)]
pub enum Command {
    Look,
    Go(String),
    North,
    South,
    East,
    West,
    Take(String),
    Drop(String),
    Inventory,
    Help,
    Unknown,
}

pub fn parse_command(input: &str) -> Command {
    let tokens: Vec<&str> = input.trim().split_whitespace().collect();
    
    if tokens.is_empty() {
        return Command::Unknown;
    }
    
    let cmd = tokens[0].to_lowercase();
    
    match cmd.as_str() {
        "look" | "l" | "examine" | "x" => Command::Look,
        "go" => {
            if tokens.len() > 1 {
                Command::Go(tokens[1].to_string())
            } else {
                Command::Unknown
            }
        },
        "n" | "north" => Command::North,
        "s" | "south" => Command::South,
        "e" | "east" => Command::East,
        "w" | "west" => Command::West,
        "take" | "get" | "pick" => {
            if tokens.len() > 1 {
                // Handle "pick up" as a special case
                if cmd == "pick" && tokens.len() > 2 && tokens[1] == "up" {
                    Command::Take(tokens[2].to_string())
                } else {
                    Command::Take(tokens[1].to_string())
                }
            } else {
                Command::Unknown
            }
        },
        "drop" => {
            if tokens.len() > 1 {
                Command::Drop(tokens[1].to_string())
            } else {
                Command::Unknown
            }
        },
        "inventory" | "i" | "inv" => Command::Inventory,
        "help" | "h" | "?" => Command::Help,
        _ => Command::Unknown,
    }
}