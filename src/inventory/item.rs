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
}
