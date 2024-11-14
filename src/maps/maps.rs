use crate::maps::tile::{TileContent};
use crate::entities::player::Player;
use crate::entities::npc::NPC;

pub struct Maps {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Vec<TileContent>>,
}

impl Maps {

    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            grid: vec![vec![TileContent::Empty; width]; height]
        }
    }
    pub fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height= height;
        self.grid = vec![vec![TileContent::Empty; width]; height];
    }

    pub fn width(&self) -> usize {
        self.grid[0].len()
    }

    pub fn height(&self) -> usize {
        self.grid.len()
    }

    pub fn in_bounds(&self, x: usize, y: usize) -> bool {
        x <= self.width && y <= self.height
    }

    pub fn is_empty(&self, x: usize, y: usize) -> bool {
        self.grid[y][x] == TileContent::Empty
    }

    pub fn find_nearby_npc(&self, player_position: (usize, usize), npcs: &Vec<NPC>) -> Option<usize> {
        npcs.iter().position(|npc| {
            let (npc_x, npc_y) = npc.position;
            let (player_x, player_y) = player_position;
            (npc_x as isize - player_x as isize).abs() <= 1 && (npc_y as isize - player_y as isize).abs() <= 1
        })
    }

    pub fn update_player_position(&mut self, x: usize, y: usize) {
        if self.in_bounds(x, y){
            self.grid[y][x] = TileContent::Player;
        }
    }

    pub fn draw(&self, player_position: (usize, usize), npcs: &Vec<NPC>) -> String {
        let mut map_string = String::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let tile: TileContent = if (x, y) == player_position {
                    TileContent::Player
                } else if npcs.iter().any(|npc| npc.position == (x, y)) {
                    TileContent::NPC
                } else {
                    TileContent::Empty
                };

                map_string.push(tile.to_char());
            }
            map_string.push('\n');
        }

        map_string
    }

    pub fn resize_to_frame(&mut self, frame_width: usize, frame_height: usize) {
        self.width = frame_width;
        self.height = frame_height;
        self.grid = vec![vec![TileContent::Empty; self.width]; self.height];
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_maps() {
        let maps = Maps::new(10, 10);
        assert_eq!(maps.width, 10);
        assert_eq!(maps.height, 10);
        assert_eq!(maps.grid.len(), 10);
        assert_eq!(maps.grid[0].len(), 10);
        assert_eq!(maps.grid[0][0], TileContent::Empty);
    }


    #[test]
    fn test_resize_maps() {
        let mut maps = Maps::new(10, 10);
        maps.resize(20, 15);
        assert_eq!(maps.width, 20);
        assert_eq!(maps.height, 15);
        assert_eq!(maps.grid.len(), 15);
        assert_eq!(maps.grid[0].len(), 20);
        assert_eq!(maps.grid[0][0], TileContent::Empty);
    }

    #[test]
    fn test_width_and_height() {
        let maps = Maps::new(8, 12);
        assert_eq!(maps.width(), 8);
        assert_eq!(maps.height(), 12);
    }

    #[test]
    fn test_in_bounds() {
        let maps = Maps::new(10, 10);
        assert!(maps.in_bounds(5, 5));
        assert!(!maps.in_bounds(10, 10));
        assert!(!maps.in_bounds(0, 10));
        assert!(!maps.in_bounds(10, 0));
    }

    #[test]
    fn test_is_empty() {
        let mut maps = Maps::new(5, 5);
        assert!(maps.is_empty(2, 2));
        maps.grid[2][2] = TileContent::Player;
        assert!(!maps.is_empty(2, 2));
    }

    #[test]
    fn test_find_nearby_npc() {
        let mut maps = Maps::new(10, 10);
        let npc1 = NPC::new("Test NPC", "You shall not pass!", (5, 5), 50, 10);
        let npc2 = NPC::new("Test NPC", "You shall not pass!", (7, 7), 50, 10);

        let npcs = vec![npc1, npc2];

        assert_eq!(maps.find_nearby_npc((5, 5), &npcs), Some(0));
        assert_ne!(maps.find_nearby_npc((3, 3), &npcs), Some(1));
        assert_eq!(maps.find_nearby_npc((1, 1), &npcs), None);
    }

    #[test]
    fn test_update_player_position() {
        let mut maps = Maps::new(10, 10);
        maps.update_player_position(5, 5);
        assert_eq!(maps.grid[5][5], TileContent::Player);

        maps.update_player_position(7, 7);
        assert_eq!(maps.grid[7][7], TileContent::Player);
    }

    #[test]
    fn test_draw() {
        let mut maps = Maps::new(10, 10);
        let mut player = Player::new("Test Player");
        let npc = NPC::new("Test NPC", "You shall not pass!", (5, 5), 50, 10);
        let npcs = vec![npc];

        let map_string = maps.draw(player.position, &npcs);
        assert!(map_string.contains("."));
        assert!(map_string.contains("@"));
        assert!(map_string.contains("N"));
    }

    #[test]
    fn test_resize_to_frame() {
        let mut maps = Maps::new(20, 20);
        maps.resize_to_frame(10, 15);
        assert_eq!(maps.width, 10);
        assert_eq!(maps.height, 15);
        assert_eq!(maps.grid.len(), 15);
        assert_eq!(maps.grid[0].len(), 10);
    }

}