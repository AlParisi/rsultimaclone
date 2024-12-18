use rand::Rng;
use crate::entities::player::Player;
use crate::entities::npc::NPC;
use crate::inventory::item::Item;
use crate::maps::maps::Maps;
use crate::ui;

pub fn start() {
    // 10X10 test map
    let mut map = Maps::new(1,1);
    // Create and position player
    let mut player = Player::new("Hero");

    // Create and position NPCs
    let npc1 = NPC::new("Guard", "You shall not pass!", (rand::thread_rng().gen_range(1..70), rand::thread_rng().gen_range(1..20)), 50, 10);
    let npc2 = NPC::new("Guard", "You shall not pass!", (rand::thread_rng().gen_range(1..70), rand::thread_rng().gen_range(1..20)), 50, 10);
    let npc3 = NPC::new("Guard", "You shall not pass!", (rand::thread_rng().gen_range(1..70), rand::thread_rng().gen_range(1..20)), 50, 10);

    let mana = Item::new("Mana Potion", (14 ,7));
    let life = Item::new("Health Potion", (10, 10));

    let mut npcs:  Vec< crate::entities::npc::NPC >  = Vec::new();
    npcs.push(npc1);
    npcs.push(npc2);
    npcs.push(npc3);
    let mut items:  Vec< crate::inventory::item::Item >  = Vec::new();
    items.push(mana);
    items.push(life);

    if let Err(err) = ui::run_ui(&mut player, &mut map, &mut npcs, &mut items) {
        println!("Error: {:?}", err);
    }

}

