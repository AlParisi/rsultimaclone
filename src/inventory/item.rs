use ratatui::layout::Positions;
use crate::entities::player::Player;

#[derive(Debug, Clone)]
pub struct Item {
    pub name: String,
    pub position: (usize, usize)
}

impl Item {
    pub fn new(name: &str, position: (usize, usize)) -> Self {
        Self {
            name: name.to_string(),
            position
        }
    }

    pub fn add_item(player: &mut Player, item: Item) -> String{
        player.inventory.push(item);
        "You found a new item!".to_string()
    }

    pub fn use_item(player: &mut Player, item_name: &str) -> String {
        if let Some(index) = player.inventory.iter().position(|item| item.name == item_name) {
            let item = player.inventory.remove(index);
            apply_item_effect(player, &item);
            "You used item".to_string()
        } else {
            "Object not found on inventary.".to_string()
        }
    }

}

fn apply_item_effect(player: &mut Player, item: &Item) {
    match item.name.as_str() {
        "Health Potion" => {
            player.health += 20;
            println!("Your health is increased by 20 points. Current health: {}.", player.health);
        },
        "Mana Potion" => {
            player.mana += 15;
            println!("Your mana increased by 15 points. Current Mana: {}.", player.mana);
        },
        _ => println!("Object not useful")
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_item() {
        let item = Item::new("Health Potion", (10, 10));
        assert_eq!(item.name, "Health Potion");
    }

    #[test]
    fn test_add_item() {
        let mut player = Player::new("Test Player");
        let item = Item::new("Health Potion", (10, 10));
        Item::add_item(&mut player, item);
        assert_eq!(player.inventory.len(), 1);
        assert_eq!(player.inventory[0].name, "Health Potion");
    }

    #[test]
    fn test_use_item() {
        let mut player = Player::new("Test Player");
        let potion = Item::new("Health Potion", (10, 10));
        Item::add_item(&mut player, potion);

        Item::use_item(&mut player, "Health Potion");
        assert_eq!(player.inventory.len(), 0);
        assert_eq!(player.health, 120);

        Item::use_item(&mut player, "Mana Potion");
        assert_eq!(player.inventory.len(), 0);
        assert_eq!(player.mana, 50);
    }

    #[test]
    fn test_apply_item_effect() {
        let mut player = Player::new("Test Player");
        let health_potion = Item::new("Health Potion", (10, 10));
        let mana_potion = Item::new("Mana Potion", (5, 5));

        apply_item_effect(&mut player, &health_potion);
        assert_eq!(player.health, 120);
        apply_item_effect(&mut player, &mana_potion);
        assert_eq!(player.mana, 65);
    }

}