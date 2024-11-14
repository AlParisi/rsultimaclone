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


impl TileContent {
    pub(crate) fn to_char(self) -> char {
        match self {
            TileContent::Player => '@',
            TileContent::NPC => 'N',
            TileContent::Empty => '.',
            TileContent::Obstacle => '#'
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_char() {
        let empty_tile = TileContent::Empty;
        assert_eq!(empty_tile.to_char(), '.');
        let player_tile = TileContent::Player;
        assert_eq!(player_tile.to_char(), '@');
        let npc_tile = TileContent::NPC;
        assert_eq!(npc_tile.to_char(), 'N');
        let obstacle_tile = TileContent::Obstacle;
        assert_eq!(obstacle_tile.to_char(), '#');
    }

}