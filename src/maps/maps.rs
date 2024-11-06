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
            grid: vec![vec![TileContent::Empty; width]; height],
        }
    }
    pub fn resize(&mut self, width: usize, height: usize) {
        self.grid = vec![vec![TileContent::Empty; width]; height];
    }

    pub fn width(&self) -> usize {
        self.grid[0].len()
    }

    pub fn height(&self) -> usize {
        self.grid.len()
    }

    fn in_bounds(&self, x: usize, y: usize) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }

    pub fn is_empty(&self, x: usize, y: usize) -> bool {
        self.grid[y][x] == TileContent::Empty
    }

    pub fn place_player(&mut self, player: &mut Player, x: usize, y: usize) {
        if self.in_bounds(x, y) {
            self.grid[y][x] = TileContent::Player;
            player.position = (x, y);
            println!("The player was placed in ({}, {}).", x, y);
        } else {
            println!("Location outside the limits of the map.");
        }
    }

    pub fn place_npc(&mut self, npc: NPC, x: usize, y: usize) {
        if self.in_bounds(x, y) {
            self.grid[y][x] = TileContent::NPC;
            println!("NPC '{}' was placed in ({}, {}).", npc.name, x, y);
        } else {
            println!("Location outside the limits of the map.");
        }
    }

    pub fn move_player(&mut self, player: &mut Player, dx: usize, dy: usize) {
        let new_x = player.position.0 + dx;
        let new_y = player.position.1 + dy;

        if self.in_bounds(new_x, new_y) {
            //Free the old position
            self.grid[player.position.1][player.position.0] = TileContent::Empty;
            //Update the new location
            self.place_player(player, new_x, new_y);
        } else {
            println!("You can't move in that direction, you're at the edge of the map.");
        }
    }

    pub fn find_nearby_npc(&self, player_position: (usize, usize), npcs: &Vec<NPC>) -> Option<usize> {
        npcs.iter().position(|npc| {
            let (npc_x, npc_y) = npc.position;
            let (player_x, player_y) = player_position;
            (npc_x as isize - player_x as isize).abs() <= 1 && (npc_y as isize - player_y as isize).abs() <= 1
        })
    }

    pub fn update_player_position(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
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
