use crate::entities::player::Player;

pub struct Item {
    pub name: String,
    pub value: i32,
}

impl Item {
    pub fn new(name: &str, value: i32) -> Self {
        Self {
            name: name.to_string(),
            value,
        }
    }

    pub fn add_item(mut player: Player, item: Item) {
        println!("You found a new item: {}.", item.name);
        player.inventory.push(item);
    }

    pub fn use_item(mut player: Player, item_name: &str) {
        if let Some(index) = player.inventory.iter().position(|item| item.name == item_name) {
            let item = player.inventory.remove(index);
            println!("You used item: {}.", item.name);
            apply_item_effect(player, &item);
        } else {
            println!("Object not found on inventary.");
        }
    }

}

fn apply_item_effect(mut player: Player, item: &Item) {
    match item.name.as_str() {
        "Health Potion" => {
            player.health += 20;
            println!("Your health is increased by 20 points. Current health: {}.", player.health);
        },
        "Mana Potion" => {
            player.mana += 15;
            println!("Your mana increased by 15 points. Current Mana: {}.", player.mana);
        },
        _ => println!("Object not useful"),
    }
}