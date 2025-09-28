# Nemu - Text Adventure Engine

A complete text-based adventure game framework in Rust. Create, play, and share interactive stories with rich narratives and inventory systems.

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

## Features

- **Room-based navigation** with text descriptions and atmospheric details
- **Inventory system** with item combinations and usage mechanics
- **Save/load game states** with multiple slots and quick save/load
- **Natural language command parsing**
- **Story files** in TOML format with scripting capabilities
- **Complex branching narratives** with flags and conditions

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

### Available Commands
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
- `look` - Look around the current room
- `go [direction]` - Move in a direction (north, south, east, west)
- `n/s/e/w` - Short forms for directions
- `take [item]` - Pick up an item
- `drop [item]` - Drop an item
- `inventory` - Check your inventory
- `help` - Show available commands
- `quit` - Exit the game

## Story Format

Stories are written in TOML format. Here's an example:

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

## Quick Start Example

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

## Development

### Building for Development
```bash
# Build in debug mode
cargo build

# Run directly
cargo run -- create test_story
cargo run -- play test_story/test_story.toml
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
    └── create.rs    # Story creation
```

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Test your changes
5. Commit your changes (`git commit -m 'Add some amazing feature'`)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

## License

MIT