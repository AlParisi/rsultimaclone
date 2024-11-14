#[derive(Debug)]
pub struct NPC {
    pub name: String,
    pub dialogue: String,
    pub position: (usize, usize),
    pub health: i32,
    pub strength: i32
}

impl NPC {
    pub fn new(name: &str, dialogue: &str, position: (usize, usize), health: i32, strength: i32) -> Self {
        Self {
            name: name.to_string(),
            dialogue: dialogue.to_string(),
            position,
            health,
            strength
        }
    }

    pub fn attack(&self) -> i32 {
        self.strength
    }

    pub fn interact(&self) -> String{
        self.dialogue.clone()
    }
}
