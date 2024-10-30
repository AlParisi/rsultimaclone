use crate::entities::player::Player;
use crate::entities::npc::NPC;
use std::io::{self, Write};
use crate::maps::maps::Maps;
use crate::maps::tile::TileContent;

pub fn start() {
    // 10X10 test map
    let mut map = Maps::new(10, 10);

    // Create and position player
    let mut player = Player::new("Hero", (10, 10));
    map.place_player(&mut player, 5, 5);

    // Create and position NPCs
    let npc1 = NPC::new("Guard", "You shall not pass!");
    let npc2 = NPC::new("Merchant", "Welcome! Do you want to buy something?");
    map.place_npc(&npc1, 2, 2);
    map.place_npc(&npc2, 7, 7);

    // game loop
    loop {
        // clean console load map
        print!("\x1B[2J\x1B[1;1H"); // ANSI code to clean terminal
        map.display_map();

        println!("Enter a command (up, down, left, right, interact, fight, train, exit): ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error reading the command");
        let input = input.trim();

        match input {
            "up" => map.move_player(&mut player, 0, -1),
            "down" => map.move_player(&mut player, 0, 1),
            "left" => map.move_player(&mut player, -1, 0),
            "right" => map.move_player(&mut player, 1, 0),
            "interact" => interact_with_npc(&player, &map, &npc1, &npc2),
            "fight" => engage_in_combat(&mut player),
            "train" => train_player(&mut player),
            "exit" => {
                println!("Thanks for playing!");
                break;
            }
            _ => println!("Invalid command."),
        }
    }
}

fn interact_with_npc(player: &Player, map: &Maps, npc1: &NPC, npc2: &NPC) {
    let (x, y) = player.position;
    match &map.grid[y as usize][x as usize].content {
        TileContent::NPC(name) if name == &npc1.name => npc1.interact(),
        TileContent::NPC(name) if name == &npc2.name => npc2.interact(),
        _ => println!("There is no one to interact with here."),
    }
}

fn engage_in_combat(player: &mut Player) {
    let enemy_health = 20;
    let player_damage = player.strength;
    let mut enemy_health = enemy_health;

    println!("An enemy attacks you! Prepare to fight.");
    while enemy_health > 0 && player.health > 0 {
        enemy_health -= player_damage;
        println!("You inflicted {} damage! The enemy's health is now {}", player_damage, enemy_health);

        if enemy_health <= 0 {
            println!("You have defeated the enemy!");
            player.gain_experience(50);
            break;
        }

        let enemy_damage = 10;
        player.health -= enemy_damage;
        println!("The enemy has inflicted on you {} damage! Your health is now {}", enemy_damage, player.health);

        if player.health <= 0 {
            println!("You have been defeated.");
            break;
        }
    }
}

fn train_player(player: &mut Player) {
    println!("You train to improve your strength and agility.");
    player.strength += 1;
    player.agility += 1;
    println!("Your strength is now {} and your agility is now {}.", player.strength, player.agility);
}
