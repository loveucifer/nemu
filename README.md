# BlockyBlocky - Interactive Story Engine

An advanced text-based adventure game framework written in Rust. Create, play, and share interactive stories with complex narratives, inventory systems, and more.

## Features

- **Room-based navigation** with rich text descriptions
- **Advanced inventory system** with item combinations and usage
- **Save/load functionality** with multiple slots
- **Natural language command parsing**
- **TOML-based story format** for easy authoring
- **Lua scripting support** for complex game logic
- **NPC dialogue systems** with personality and memory
- **Combat mechanics** for action-adventure games
- **SQLite-based save system** for persistent game states
- **Achievement tracking** with progress and unlockables
- **Plugin architecture** for extensibility
- **Colorful terminal UI** with animations
- **Command history and tab completion**
- **Story creation tools and templates**

## Installation

```bash
# Clone the repository
git clone https://github.com/blockyblocky/engine.git
cd engine

# Build the project
cargo build --release

# The executable will be available as 'bby' in the target/release directory
```

## CLI Usage

```bash
# Create a new story
bby create my-adventure

# Play a story
bby play story.toml

# Validate a story file
bby validate story.toml

# Continue last game
bby continue

# Load a saved game
bby load slot1

# Export to different format
bby export story.toml --format web

# Test a story with automation
bby test story.toml

# Lint a story for best practices
bby lint story.toml
```

## Story Format

Stories are written in TOML format with support for complex narratives:

```toml
[story]
title = "The Quantum Lighthouse"
author = "Your Name"
version = "1.0.0"
engine_version = ">=1.0.0"
tags = ["sci-fi", "puzzle", "exploration"]

[config]
auto_save_interval = 300  # seconds
max_inventory_weight = 100
enable_combat = true
lua_scripting = true

[rooms.lab]
title = "Quantum Research Lab"
description = """
Holographic displays flicker with incomprehensible equations. The air
hums with barely contained energy from the quantum field generator.
A coffee mug sits abandoned on a desk - still warm.
"""
atmosphere = "mysterious"
lighting = "dim_blue"
exits = { north = "corridor", up = "observation_deck" }
items = ["quantum_stabilizer", "research_notes", "coffee_mug"]
npcs = ["dr_chen"]

[rooms.lab.on_enter]
script = """
if player.has_flag("first_visit_lab") then
    game.show_message("You've been here before...")
else
    game.set_flag("first_visit_lab", true)
    game.show_message("The lab feels familiar yet alien.")
end
"""

[items.quantum_stabilizer]
name = "Quantum Field Stabilizer"
description = "A sleek device that pulses with ethereal light."
weight = 5
tags = ["tool", "quantum", "valuable"]
use_script = "quantum_mechanics.lua"

[npcs.dr_chen]
name = "Dr. Sarah Chen"
personality = "brilliant_but_scattered"
memory_span = 10  # remembers last 10 interactions
dialogue_tree = "chen_conversations.toml"

[achievements.first_steps]
id = "first_steps"
name = "First Steps"
description = "Start your first adventure"
secret = false
points = 10

[plugins]
enabled = ["weather", "time"]
```

## Architecture

BlockyBlocky follows a modular architecture:

- `core/` - Core data structures and game logic
- `game/` - Main game engine and state management
- `cli/` - Command-line interface and user commands
- `story/` - Story parsing and validation
- `save/` - Save/load functionality with SQLite
- `ui/` - Terminal user interface with colors and input handling

### Core Components

#### Game Engine
The main game engine handles command processing, state updates, and game flow. It implements the `GameEngine` trait which defines standard operations like processing commands, updating the game state, and managing saves.

#### Story Format
Stories are defined in TOML files with a hierarchical structure supporting:
- Rooms with descriptions and navigation
- Items with properties and scripting
- NPCs with dialogue trees and personalities
- Achievements and game progression
- Lua scripts for complex logic

#### Scripting System
Lua scripting is integrated for complex game logic that goes beyond the built-in mechanics. Scripts can modify game state, show messages, and interact with the player.

#### Achievement System
Achievements are tracked through an event-based system that responds to player actions:
- Room exploration
- Item collection
- Combat victories
- Puzzle completion
- Story progression

#### Plugin Architecture
The plugin system allows for extensibility with features like:
- Weather systems
- Time/day cycles
- Custom gameplay mechanics
- Enhanced UI elements

## Development

To contribute to BlockyBlocky:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Run `cargo test` to ensure everything works
6. Submit a pull request

### Building from Source

```bash
# Clone the repository
git clone https://github.com/blockyblocky/engine.git
cd engine

# Build in debug mode
cargo build

# Build in release mode
cargo build --release

# Run tests
cargo test

# Format code
cargo fmt

# Check for linting issues
cargo clippy
```

## License

MIT

## Contributing

We welcome contributions to BlockyBlocky! Please read our [Contributing Guide](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.

## Acknowledgments

- The Rust community for the excellent ecosystem
- TOML authors for the clean configuration format
- Crossterm for terminal manipulation
- MLua for Lua integration
- All open-source libraries that made this project possible

## Support

For support, please open an issue in the GitHub repository or contact the maintainers.