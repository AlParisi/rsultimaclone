use crate::maps::maps::tile::{TileContent, Maps};
use crate::entities::player::Player;
use crate::entities::npc::NPC;

#[test]
fn test_new_maps() {
    let maps = Maps::new(10, 10);
    assert_eq!(maps.width, 10);
    assert_eq!(maps.height, 10);
    assert_eq!(maps.grid.len(), 10);
    assert_eq!(maps.grid[0].len(), 10);
    assert_eq!(maps.grid[0][0], TileContent::Empty);
}
