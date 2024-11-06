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

#[derive(Debug)]
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
