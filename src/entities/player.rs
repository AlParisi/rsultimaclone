use crate::entities::npc::NPC;
use crate::inventory::item::Item;
use crate::maps::maps::Maps;

pub struct Player {
    pub name: String,
    pub health: i32,
    pub mana: i32,
    pub level: u32,
    pub experience: u32,
    pub strength: i32,
    pub agility: i32,
    pub charisma: i32,
    pub inventory: Vec<Item>,
    pub position: (usize, usize),
    pub map_limits: (usize, usize),
    pub status: PlayerStatus
}

#[derive(Debug, PartialEq)]
pub enum PlayerStatus {
    Normal,
    InCombat,
    Exhausted,
    Injured,
}
impl Player {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            health: 100,
            mana: 50,
            level: 1,
            experience: 0,
            strength: 10,
            agility: 8,
            charisma: 5,
            inventory: Vec::new(),
            position: (1, 1),
            map_limits: (100, 100),
            status: PlayerStatus::Normal,
        }
    }

    pub fn move_player(&mut self, dx: usize, dy: usize) {
        let new_x = self.position.0 + dx;
        let new_y = self.position.1 + dy;

        if new_x < 0 || new_x >= self.map_limits.0 || new_y < 0 || new_y >= self.map_limits.1 {
            println!("You can't move in that direction, you're at the edge of the maps.");
        } else {
            self.position = (new_x, new_y);
            println!("You moved to the position: ({}, {})", self.position.0, self.position.1);
        }
    }

    pub fn gain_experience(&mut self, amount: u32) {
        self.experience += amount;
        println!("You gained {} point experience.", amount);

        let required_experience = self.level * 100;
        if self.experience >= required_experience {
            self.level_up();
        }
    }

    fn level_up(&mut self) {
        self.level += 1;
        self.experience = 0;
        self.health += 20;
        self.mana += 10;
        self.strength += 2;
        self.agility += 1;
        self.charisma += 1;
        println!("Congratulation! You achieved a level up: now your level is {}.", self.level);
    }

    pub fn set_status(&mut self, status: PlayerStatus) {
        self.status = status;
        println!("Your current status is: {:?}", self.status);
    }

    pub fn check_status(&self) {
        match self.status {
            PlayerStatus::Normal => println!("You are in good condition"),
            PlayerStatus::InCombat => println!("You are in a fight!"),
            PlayerStatus::Exhausted => println!("You are exhausted and need rest."),
            PlayerStatus::Injured => println!("You are hurt and you need to heal."),
        }
    }

    fn attack(&self) -> i32 {
        self.strength
    }

    pub fn engage_in_combat(&mut self, npc: &mut NPC) -> String{
        let mut combat_log = String::new();

        loop {
            // player attack
            let player_damage = self.attack();
            npc.health -= player_damage;
            combat_log.push_str(&format!("You dealt {} damage to the NPC!\n", player_damage));

            if npc.health <= 0 {
                combat_log.push_str("You defeated the NPC!\n");
                break;
            }

            // NPC attack
            let npc_damage = npc.attack();
            self.health -= npc_damage;
            combat_log.push_str(&format!("The NPC dealt {} damage to you!\n", npc_damage));

            if self.health <= 0 {
                combat_log.push_str("You were defeated by the NPC!\n");
                break;
            }
        }

        combat_log
    }

    pub fn train_player(player: &mut Player) {
        println!("You train to improve your strength and agility.");
        player.strength += 1;
        player.agility += 1;
        println!("Your strength is now {} and your agility is now {}.", player.strength, player.agility);
    }

    pub fn move_up(&mut self, map: &Maps) {
        if self.position.1 > 0 && map.is_empty(self.position.0, self.position.1 - 1) {
            self.position.1 -= 1;
        }
    }

    pub fn move_down(&mut self, map: &Maps) {
        if self.position.1 < map.height() - 1 && map.is_empty(self.position.0, self.position.1 + 1) {
            self.position.1 += 1;
        }
    }

    pub fn move_left(&mut self, map: &Maps) {
        if self.position.0 > 0 && map.is_empty(self.position.0 - 1, self.position.1) {
            self.position.0 -= 1;
        }
    }

    pub fn move_right(&mut self, map: &Maps) {
        if self.position.0 < map.width() - 1 && map.is_empty(self.position.0 + 1, self.position.1) {
            self.position.0 += 1;
        }
    }
}


mod tests {
    use super::*;

    #[test]
    fn test_new_player() {
        let player = Player::new("Test Player");
        assert_eq!(player.name, "Test Player");
        assert_eq!(player.health, 100);
        assert_eq!(player.mana, 50);
        assert_eq!(player.level, 1);
        assert_eq!(player.experience, 0);
        assert_eq!(player.strength, 10);
        assert_eq!(player.agility, 8);
        assert_eq!(player.charisma, 5);
        assert_eq!(player.inventory.len(), 0);
        assert_eq!(player.position, (1, 1));
        assert_eq!(player.map_limits, (100, 100));
    }

    #[test]
    fn test_move_player() {
        let mut player = Player::new("Test Player");
        player.move_player(1, 1);
        assert_eq!(player.position, (2, 2));

        player.move_player(100, 100);
        assert_ne!(player.position, (102, 102));
    }

    #[test]
    fn test_gain_experience() {
        let mut player = Player::new("Test Player");
        player.gain_experience(50);
        assert_eq!(player.experience, 50);
        assert_eq!(player.level, 1);

        player.gain_experience(50);
        assert_eq!(player.experience, 0);
        assert_eq!(player.level, 2);
        assert_eq!(player.health, 120);
        assert_eq!(player.mana, 60);
        assert_eq!(player.strength, 12);
        assert_eq!(player.agility, 9);
        assert_eq!(player.charisma, 6);
    }

    #[test]
    fn test_set_status() {
        let mut player = Player::new("Test Player");
        player.set_status(PlayerStatus::InCombat);
        assert_eq!(player.status, PlayerStatus::InCombat);

        player.set_status(PlayerStatus::Exhausted);
        assert_eq!(player.status, PlayerStatus::Exhausted);
    }

    #[test]
    fn test_check_status() {
        let mut player = Player::new("Test Player");
        player.set_status(PlayerStatus::Normal);
        player.check_status();

        player.set_status(PlayerStatus::InCombat);
        player.check_status();

        player.set_status(PlayerStatus::Exhausted);
        player.check_status();

        player.set_status(PlayerStatus::Injured);
        player.check_status();
    }

    #[test]
    fn test_engage_in_combat() {
        let mut player = Player::new("Test Player");
        let mut npc = NPC::new("Test NPC", "You shall not pass!", (5, 5), 50);

        let combat_log = player.engage_in_combat(&mut npc);
        assert!(combat_log.contains("You dealt"));
        assert!(combat_log.contains("The NPC dealt"));

        assert!(player.health <= 100);
        assert!(npc.health <= 50);
    }

    #[test]
    fn test_train_player() {
        let mut player = Player::new("Test Player");
        let original_strength = player.strength;
        let original_agility = player.agility;

        Player::train_player(&mut player);
        assert_eq!(player.strength, original_strength + 1);
        assert_eq!(player.agility, original_agility + 1);
    }

    #[test]
    fn test_move_up_down_left_right() {
        let mut player = Player::new("Test Player");
        let mut maps = Maps::new(10, 10);

        // Move up
        player.position = (5, 5);
        player.move_up(&maps);
        assert_eq!(player.position, (5, 4));

        // Move down
        player.move_down(&maps);
        assert_eq!(player.position, (5, 5));

        // Move left
        player.move_left(&maps);
        assert_eq!(player.position, (4, 5));

        // Move right
        player.move_right(&maps);
        assert_eq!(player.position, (5, 5));
    }
}