use crate::entities::player::Player;
use crate::entities::npc::NPC;
use crate::maps::maps::Maps;
use crate::ui;

pub fn start() {
    // 10X10 test map
    let mut map = Maps::new(1,1);
    // Create and position player
    let mut player = Player::new("Hero");

    // Create and position NPCs
    let npc1 = NPC::new("Guard", "You shall not pass!", (5, 5), 50);

    let mut npcs:  Vec< crate::entities::npc::NPC >  = Vec::new();
    npcs.push(npc1);


    if let Err(err) = ui::run_ui(&mut player, &mut map, &mut npcs) {
        println!("Error: {:?}", err);
    }

}

