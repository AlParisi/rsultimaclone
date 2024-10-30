#[derive(Debug)]
pub enum TileContent {
    Empty,
    Player,
    NPC(String), // NPC name
}

pub struct Tile {
    pub content: TileContent,
    pub description: String,
}

impl Tile {
    pub fn new(description: &str) -> Self {
        Self {
            content: TileContent::Empty,
            description: description.to_string(),
        }
    }
}
