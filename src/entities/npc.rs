pub struct NPC {
    pub name: String,
    pub dialogue: String,
}

impl NPC {
    pub fn new(name: &str, dialogue: &str) -> Self {
        Self {
            name: name.to_string(),
            dialogue: dialogue.to_string(),
        }
    }

    pub fn interact(&self) {
        println!("{} say: {}", self.name, self.dialogue);
    }
}
