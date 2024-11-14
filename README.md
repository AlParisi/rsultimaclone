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

## Basic Interface and Command Functionality

### Ratatui-Based Interface

We’ve integrated a basic UI for the game using the [Ratatui](https://ratatui.rs) library to improve the visual experience in the command-line console. The interface layout is divided into three main sections:

1. **Map Display**: The main view, showing the player's location, NPCs, and other map details. This area dynamically adjusts its size based on the terminal window to fit available space, ensuring an optimal view of the player’s surroundings.

2. **Player Stats Panel**: Located to the right of the map, this panel displays the player's name, health, strength, agility, and experience, providing an at-a-glance summary of the player’s status.

3. **Command Window**: Located at the bottom, this area allows the player to interact with the game through commands. It displays log messages and accepts specific commands for advanced interactions, like engaging in combat.

### Real-Time Command Functionality

To improve the gameplay experience, we’ve implemented real-time movement commands as well as dedicated command handling for combat actions. Here’s how commands work:

- **Movement**: The player can move in real-time by pressing the `w`, `a`, `s`, `d` keys without needing to hit `Enter`. Each keypress immediately updates the player's position on the map.

- **Combat Commands**: Specific commands like `engage` and `fight` are entered in the Command Window and require pressing `Enter` to execute. These commands enable the player to interact with NPCs, start fights, and engage in combat.

This structure ensures a smoother gameplay experience, with instant responses to movement inputs while preserving the Command Window for complex interactions.


### Screen
```console
┌Map──────────────────────────────────────────────────────────────────────────────────────────────┐┌Player Stats─────────────────────────────┐
│.................................................................................................││Name: Hero                               │
│.................................................................................................││Health: 100                              │
│.................................................................................................││Strength: 10                             │
│.................................................................................................││Agility: 8                               │
│.................................................................................................││Experience: 0                            │
│.................................................................................................││                                         │
│.................................................................................................││                                         │
│..............I..................................................................................││                                         │
│.................................................................................................││                                         │
│.................................................................................................│└─────────────────────────────────────────┘
│..........I......................................................................................│┌Inventary────────────────────────────────┐
│.................................................................................................││1. Mana Potion                           │
│.................................................................................................││2. Health Potion                         │
│.................................................................................................││                                         │
│.................@..............................................N................................││                                         │
│.................................................................................................││                                         │
│................................................N................................................││                                         │
│.................................................................................................││                                         │
│.......................................N.........................................................││                                         │
│.................................................................................................││                                         │
└─────────────────────────────────────────────────────────────────────────────────────────────────┘└─────────────────────────────────────────┘
┌Commands────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┐
│> Player moved down                                                                                                                         │
│                                                                                                                                            │
│                                                                                                                                            │
└────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘
```



## Running the Project
To start the project:
1. Clone the repository.
2. Make sure Rust is installed. If not, you can install it from [rust-lang.org](https://www.rust-lang.org/).
3. Run the game with the command:
   ```bash
   cargo run
