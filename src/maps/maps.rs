use crate::maps::tile::{Tile, TileContent};
use crate::entities::player::Player;
use crate::entities::npc::NPC;

pub struct Maps {
    pub width: i32,
    pub height: i32,
    pub grid: Vec<Vec<Tile>>,
}

impl Maps {

    pub fn new(width: i32, height: i32) -> Self {
        let grid = (0..height).map(|_| {
            (0..width).map(|_| Tile::new("An empty lot.")).collect()
        }).collect();

        Self { width, height, grid }
    }

    pub fn place_player(&mut self, player: &mut Player, x: i32, y: i32) {
        if self.in_bounds(x, y) {
            self.grid[y as usize][x as usize].content = TileContent::Player;
            player.position = (x, y);
            println!("The player was placed in ({}, {}).", x, y);
        } else {
            println!("Location outside the limits of the map.");
        }
    }

    pub fn place_npc(&mut self, npc: &NPC, x: i32, y: i32) {
        if self.in_bounds(x, y) {
            self.grid[y as usize][x as usize].content = TileContent::NPC(npc.name.clone());
            println!("NPC '{}' was placed in ({}, {}).", npc.name, x, y);
        } else {
            println!("Location outside the limits of the map.");
        }
    }

    fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }

    pub fn move_player(&mut self, player: &mut Player, dx: i32, dy: i32) {
        let new_x = player.position.0 + dx;
        let new_y = player.position.1 + dy;

        if self.in_bounds(new_x, new_y) {
            //Free the old position
            self.grid[player.position.1 as usize][player.position.0 as usize].content = TileContent::Empty;
            //Update the new location
            self.place_player(player, new_x, new_y);
        } else {
            println!("You can't move in that direction, you're at the edge of the map.");
        }
    }

    pub fn display_map(&self) {
        for row in &self.grid {
            for tile in row {
                match &tile.content {
                    TileContent::Empty => print!(". "),
                    TileContent::Player => print!("P "),
                    TileContent::NPC(name) => print!("N ")
                }
            }
            println!();
        }
    }
}
