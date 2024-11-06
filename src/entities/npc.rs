pub struct NPC {
    pub name: String,
    pub dialogue: String,
    pub position: (usize, usize),
    pub health: i32,
    pub strength: i32
}

impl NPC {
    pub fn new(name: &str, dialogue: &str, position: (usize, usize), strength: i32) -> Self {
        Self {
            name: name.to_string(),
            dialogue: dialogue.to_string(),
            position,
            health: 30,
            strength
        }
    }

    pub fn attack(&self) -> i32 {
        self.strength
    }

    pub fn interact(&self) {
        println!("{} say: {}", self.name, self.dialogue);
    }
}
