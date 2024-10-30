# Ultima Online Console Clone in Rust

A simple **Ultima Online** clone designed for the console, developed in **Rust** to explore the basics of the language building a retro-style command-line game programming.

This project simulates some essential mechanics of a grid-based RPG game, with a **player** who moves on a map, interacts with **NPCs** (non-playable characters), and trains to increase their stats. The game doesn’t have a predefined ending but includes some simple interactions and basic functionalities, and is designed to be extendable.

## Features
- **Map Movement**: the player can move in four directions within the map boundaries.
- **NPC Interaction**: some NPCs are placed on the map and can have specific dialogues with the player.
- **Combat**: the player can engage an enemy in combat, dealing and receiving damage in turns.
- **Training**: the player can increase their stats (strength and agility) through training.

## Project Structure
The project is organized into folders for easy code management and extension:

- **src/main.rs**: the main entry point of the application, which invokes the game loop.
- **src/game.rs**: contains the main game loop and manages player inputs.
- **src/entities**: contains the main game entities.
    - `player.rs`: defines the structure and functionality of the player character.
    - `npc.rs`: defines the structure and functionality of NPCs.
- **src/map**: contains code for managing the map.
    - `tile.rs`: defines the contents of each map tile (empty, player, NPC).
    - `maps.rs`: manages the map structure, positioning, and movement of entities.

## Running the Project
To start the project:
1. Clone the repository.
2. Make sure Rust is installed. If not, you can install it from [rust-lang.org](https://www.rust-lang.org/).
3. Run the game with the command:
   ```bash
   cargo run
