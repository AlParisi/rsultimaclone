pub struct Quest {
    pub title: String,
    pub description: String,
}

impl Quest {
    pub fn new(title: &str, description: &str) -> Self {
        Self {
            title: title.to_string(),
            description: description.to_string(),
        }
    }

    pub fn start(&self) {
        println!("Quest started: {}", self.title);
    }
}
