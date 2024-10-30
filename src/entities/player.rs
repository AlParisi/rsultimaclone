use crate::inventory::item::Item;

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
    pub position: (i32, i32),
    pub map_limits: (i32, i32),
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
    pub fn new(name: &str, x: (i32, i32)) -> Self {
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
            position: (0, 0),
            map_limits: (100, 100),
            status: PlayerStatus::Normal,
        }
    }

    pub fn move_player(&mut self, dx: i32, dy: i32) {
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

    pub fn add_item(&mut self, item: Item) {
        println!("You found a new item: {}.", item.name);
        self.inventory.push(item);
    }

    pub fn use_item(&mut self, item_name: &str) {
        if let Some(index) = self.inventory.iter().position(|item| item.name == item_name) {
            let item = self.inventory.remove(index);
            println!("You used item: {}.", item.name);
            self.apply_item_effect(&item);
        } else {
            println!("Object not found on inventary.");
        }
    }

    fn apply_item_effect(&mut self, item: &Item) {
        match item.name.as_str() {
            "Health Potion" => {
                self.health += 20;
                println!("Your health is increased by 20 points. Current health: {}.", self.health);
            },
            "Mana Potion" => {
                self.mana += 15;
                println!("Your mana increased by 15 points. Current Mana: {}.", self.mana);
            },
            _ => println!("Object not useful"),
        }
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
}
