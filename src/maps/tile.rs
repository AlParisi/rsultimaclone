#[derive(Debug, Clone, PartialEq)]
pub enum TileContent {
    Empty,
    Player,
    NPC,
    Obstacle
}

pub struct Tile {
    pub content: TileContent,
    pub description: String
}

impl Tile {
    pub fn new(description: &str) -> Self {
        Self {
            content: TileContent::Empty,
            description: description.to_string()
        }
    }
}

impl TileContent {
    pub(crate) fn to_char(self) -> char {
        match self {
            TileContent::Player => '@',
            TileContent::NPC => 'N',
            TileContent::Empty => '.',
            TileContent::Obstacle => '#', // Carattere per eventuali ostacoli
        }
    }
}
