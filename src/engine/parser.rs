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
    Save(String),
    Load(String),
    ListSaves,
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
        "save" => {
            if tokens.len() > 1 {
                Command::Save(tokens[1].to_string())
            } else {
                Command::Save("savegame.save".to_string()) // Default save name
            }
        },
        "load" => {
            if tokens.len() > 1 {
                Command::Load(tokens[1].to_string())
            } else {
                Command::Load("savegame.save".to_string()) // Default load name
            }
        },
        "saves" | "list" => Command::ListSaves,
        "help" | "h" | "?" => Command::Help,
        _ => Command::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_look_commands() {
        assert!(matches!(parse_command("look"), Command::Look));
        assert!(matches!(parse_command("l"), Command::Look));
        assert!(matches!(parse_command("examine"), Command::Look));
        assert!(matches!(parse_command("x"), Command::Look));
    }

    #[test]
    fn test_parse_direction_commands() {
        assert!(matches!(parse_command("n"), Command::North));
        assert!(matches!(parse_command("north"), Command::North));
        assert!(matches!(parse_command("s"), Command::South));
        assert!(matches!(parse_command("south"), Command::South));
        assert!(matches!(parse_command("e"), Command::East));
        assert!(matches!(parse_command("east"), Command::East));
        assert!(matches!(parse_command("w"), Command::West));
        assert!(matches!(parse_command("west"), Command::West));
    }

    #[test]
    fn test_parse_move_commands() {
        if let Command::Go(direction) = parse_command("go north") {
            assert_eq!(direction, "north");
        } else {
            panic!("Expected Go command");
        }

        if let Command::Go(direction) = parse_command("go east") {
            assert_eq!(direction, "east");
        } else {
            panic!("Expected Go command");
        }
    }

    #[test]
    fn test_parse_take_commands() {
        if let Command::Take(item) = parse_command("take key") {
            assert_eq!(item, "key");
        } else {
            panic!("Expected Take command");
        }

        if let Command::Take(item) = parse_command("get sword") {
            assert_eq!(item, "sword");
        } else {
            panic!("Expected Take command");
        }

        if let Command::Take(item) = parse_command("pick up coin") {
            assert_eq!(item, "coin");
        } else {
            panic!("Expected Take command");
        }
    }

    #[test]
    fn test_parse_drop_command() {
        if let Command::Drop(item) = parse_command("drop key") {
            assert_eq!(item, "key");
        } else {
            panic!("Expected Drop command");
        }
    }

    #[test]
    fn test_parse_inventory_commands() {
        assert!(matches!(parse_command("inventory"), Command::Inventory));
        assert!(matches!(parse_command("i"), Command::Inventory));
        assert!(matches!(parse_command("inv"), Command::Inventory));
    }

    #[test]
    fn test_parse_save_commands() {
        if let Command::Save(filename) = parse_command("save mygame.save") {
            assert_eq!(filename, "mygame.save");
        } else {
            panic!("Expected Save command");
        }

        // Test default save name
        if let Command::Save(filename) = parse_command("save") {
            assert_eq!(filename, "savegame.save");
        } else {
            panic!("Expected Save command with default name");
        }
    }

    #[test]
    fn test_parse_load_commands() {
        if let Command::Load(filename) = parse_command("load mygame.save") {
            assert_eq!(filename, "mygame.save");
        } else {
            panic!("Expected Load command");
        }

        // Test default load name
        if let Command::Load(filename) = parse_command("load") {
            assert_eq!(filename, "savegame.save");
        } else {
            panic!("Expected Load command with default name");
        }
    }

    #[test]
    fn test_parse_list_saves_command() {
        assert!(matches!(parse_command("saves"), Command::ListSaves));
        assert!(matches!(parse_command("list"), Command::ListSaves));
    }

    #[test]
    fn test_parse_help_command() {
        assert!(matches!(parse_command("help"), Command::Help));
        assert!(matches!(parse_command("h"), Command::Help));
        assert!(matches!(parse_command("?"), Command::Help));
    }

    #[test]
    fn test_parse_unknown_command() {
        assert!(matches!(parse_command("unknown"), Command::Unknown));
        assert!(matches!(parse_command(""), Command::Unknown));
        assert!(matches!(parse_command("xyz"), Command::Unknown));
    }
}