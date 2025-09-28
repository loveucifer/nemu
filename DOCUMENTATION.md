# Nemu - Text Adventure Engine

A complete text-based adventure game framework in Rust. Create, play, and share interactive stories with rich narratives and inventory systems.

## Table of Contents
- [Features](#features)
- [Quick Start](#quick-start)
- [Installation](#installation)
- [Usage](#usage)
- [Story Format](#story-format)
- [Commands Reference](#commands-reference)
- [CLI Commands](#cli-commands)
- [Development](#development)
- [Testing](#testing)
- [Example](#example)

## Features

- **Room-based navigation** with text descriptions and atmospheric details
- **Inventory system** with item combinations and usage mechanics
- **Save/load game states** with multiple slots and quick save/load
- **Natural language command parsing**
- **Story files** in TOML format with scripting capabilities
- **Complex branching narratives** with flags and conditions
- **Colorful terminal UI** with crossterm for enhanced experience

## Quick Start

1. **Build the project:**
   ```bash
   cargo build --release
   ```

2. **Create a new adventure:**
   ```bash
   ./target/release/nemu create my_adventure
   ```

3. **Play the adventure:**
   ```bash
   ./target/release/nemu play my_adventure/my_adventure.toml
   ```

4. **Try in-game commands like:**
   - `look` - examine your surroundings
   - `go north` - move in a direction
   - `take key` - pick up items
   - `inventory` - check your items
   - `save` - save your game
   - `load` - load a saved game

## Installation

### Prerequisites
- Rust (latest stable)

### Build from Source
```bash
# Clone the repository
git clone https://github.com/yourusername/nemu.git
cd nemu

# Build the project
cargo build --release

# The binary will be available as 'nemu' in target/release/
```

## Usage

### Available CLI Commands
```bash
# Create a new story template
./target/release/nemu create my-adventure

# Validate a story file
./target/release/nemu validate my-adventure.toml

# Play a story
./target/release/nemu play my-adventure.toml
```

### In-Game Commands
Once playing a story, use these commands:

| Command | Description |
|---------|-------------|
| `look` or `l` or `examine` or `x` | Look around the current room |
| `go [direction]` | Move in a direction (north, south, east, west) |
| `n` or `north` | Move north |
| `s` or `south` | Move south |
| `e` or `east` | Move east |
| `w` or `west` | Move west |
| `take [item]` or `get [item]` or `pick up [item]` | Pick up an item |
| `drop [item]` | Drop an item |
| `inventory` or `i` or `inv` | Check your inventory |
| `save [filename]` | Save game to specific file or default |
| `load [filename]` | Load game from specific file or default |
| `saves` | List available save files |
| `help` or `h` or `?` | Show available commands |
| `quit` or `exit` or `q` | Exit the game |

## Story Format

Stories are written in TOML format. Here's the basic structure:

```toml
[story]
title = "The Mysterious Forest"
start_room = "forest_entrance"

[rooms.forest_entrance]
title = "Dark Forest Entrance"
description = "You stand at the edge of a dark forest. Ancient trees loom overhead."
exits = { north = "deep_forest", east = "clearing" }
items = ["stick"]

[rooms.deep_forest]
title = "Deep in the Forest"
description = "Thick branches block most sunlight. You hear mysterious sounds."
exits = { south = "forest_entrance" }
items = ["mushroom", "berries"]

[items.stick]
name = "wooden stick"
description = "A sturdy stick that might be useful."

[items.mushroom]
name = "strange mushroom"
description = "A glowing mushroom that seems to pulse with light."

[items.berries]
name = "red berries"
description = "Small red berries that smell sweet."
```

### Advanced Story Format

For more complex stories like the example provided:

```toml
[story]
title = "The Quantum Lighthouse"
author = "Nemu Creator"
version = "1.2.0"
engine_version = ">=1.0.0"
tags = ["sci-fi", "puzzle", "exploration"]

[config]
auto_save_interval = 300  # seconds
max_inventory_weight = 100
enable_combat = false
lua_scripting = true

# Rooms with exits and items
[rooms.start]
title = "Mysterious Shore"
description = """
You stand on a desolate shore, waves lapping at your feet.
A lighthouse stands in the distance, its light pulsing with an otherworldly glow.
Seaweed and shells are scattered across the wet sand.
"""
atmosphere = "mysterious"
lighting = "dim_blue"
exits = { north = "beach", east = "lighthouse_approach" }
items = ["seashell", "driftwood"]
npcs = ["old_man"]

# Items with properties
[items.seashell]
name = "Iridescent Seashell"
description = "A beautiful shell that seems to shimmer with inner light."
weight = 1
tags = ["collectible", "shiny"]
```

## CLI Commands

### nemu create <name>
Creates a new story template with the given name. This generates a directory with a starter TOML file.

```bash
# Creates my_adventure/my_adventure.toml
nemu create my_adventure
```

### nemu play <file>
Starts a game using the specified story file.

```bash
nemu play my_story.toml
```

### nemu validate <file>
Validates a story file without playing it.

```bash
nemu validate my_story.toml
```

## Development

### Building for Development
```bash
# Build in debug mode
cargo build

# Run directly
cargo run -- create my_adventure
# Play included examples:
cargo run -- play haunted_house.toml
cargo run -- play ancient_temple.toml
```

### Project Structure
```
src/
├── main.rs           # CLI entry point
├── lib.rs           # Library exports
├── engine/
│   ├── mod.rs       # Game engine module
│   ├── game.rs      # Main game state
│   ├── room.rs      # Room management
│   ├── item.rs      # Item system
│   └── parser.rs    # Command parsing
├── story/
│   ├── mod.rs       # Story loading
│   └── loader.rs    # TOML parser
└── cli/
    ├── mod.rs       # CLI commands
    ├── play.rs      # Play command
    ├── create.rs    # Story creation
    └── validate.rs  # Story validation
```

## Testing

The project includes comprehensive unit tests for all modules:

```bash
# Run all tests
cargo test

# Run tests for a specific module
cargo test engine::game
cargo test engine::parser
cargo test story::loader
```

### Test Coverage
- Game engine state management
- Command parsing functionality
- Item and room creation/movement
- Save/load functionality
- Story validation and loading
- CLI command handling

## Example

### Quick Start Example

1. **Create a new story:**
   ```bash
   cargo run -- create my_adventure
   ```

2. **Look at the generated story:**
   ```bash
   cat my_adventure/my_adventure.toml
   ```

3. **Play the story:**
   ```bash
   cargo run -- play my_adventure/my_adventure.toml
   ```

4. **In the game, try:**
   - `look` - to see your surroundings
   - `go north` - to move to another room
   - `take key` - to pick up an item
   - `inventory` - to see what you're carrying
   - `save` - to save your progress
   - `load` - to load a saved game

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Test your changes
5. Commit your changes (`git commit -m 'Add some amazing feature'`)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

## Dependencies

- `serde` - Serialization and deserialization
- `toml` - TOML file parsing for stories
- `crossterm` - Cross-platform terminal manipulation
- `clap` - Command line argument parsing
- `serde_json` - JSON save/load functionality
- `anyhow` - Error handling
- `tokio` - Async runtime

## License

MIT

## Version

1.0.0 - Ready for production use

---

Nemu - Text Adventure Engine
(c) 2024 Nemu Contributors