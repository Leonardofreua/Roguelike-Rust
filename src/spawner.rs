use super::{
    BlocksTile, CombatStats, Item, Monster, Name, Player, Position, Potion, Rect, Renderable,
    Viewshed,
};
use crate::constants::{MAP_WIDTH, VISIBLE_TILES_RANGE};
use rltk::{RandomNumberGenerator, RGB};
use specs::prelude::*;

const MAX_MONSTERS: i32 = 4;
const MAX_ITEMS: i32 = 2;
const PLAYER_ORDER: i32 = 0;
const MONSTER_ORDER: i32 = 1;
const ITEM_ORDER: i32 = 2;

/// Spawns the player and returns his/her entity object.
pub fn player(ecs: &mut World, x: i32, y: i32) -> Entity {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
            render_order: PLAYER_ORDER,
        })
        .with(Player {})
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: VISIBLE_TILES_RANGE,
            dirty: true,
        })
        .with(Name {
            name: "Player".to_string(),
        })
        .with(CombatStats {
            max_hp: 30,
            hp: 30,
            defense: 2,
            power: 5,
        })
        .build()
}

fn build_spawn_points_by_max_amount(ecs: &mut World, room: &Rect, max_amount: i32) -> Vec<usize> {
    let mut spawn_points: Vec<usize> = Vec::new();
    let mut rng = ecs.write_resource::<RandomNumberGenerator>();
    let num_artifacts = rng.roll_dice(1, max_amount + 2) - 3;

    for _i in 0..num_artifacts {
        let mut added = false;
        while !added {
            let x = (room.x1 + rng.roll_dice(1, i32::abs(room.x2 - room.x1))) as usize;
            let y = (room.y1 + rng.roll_dice(1, i32::abs(room.y2 - room.y1))) as usize;
            let idx = (y * MAP_WIDTH) + x;
            if !spawn_points.contains(&idx) {
                spawn_points.push(idx);
                added = true;
            }
        }
    }
    spawn_points
}

/// Fills a room with stuff!
pub fn spawn_room(ecs: &mut World, room: &Rect) {
    let monster_spawn_points = build_spawn_points_by_max_amount(ecs, room, MAX_MONSTERS);
    let item_spawn_points = build_spawn_points_by_max_amount(ecs, room, MAX_ITEMS);

    // Actually spawn the monsters
    for idx in monster_spawn_points.iter() {
        let x = *idx % MAP_WIDTH;
        let y = *idx / MAP_WIDTH;
        random_monster(ecs, x as i32, y as i32);
    }

    // Actually spawn the potions
    for idx in item_spawn_points.iter() {
        let x = *idx % MAP_WIDTH;
        let y = *idx / MAP_WIDTH;
        health_potion(ecs, x as i32, y as i32);
    }
}

/// Spawns a random monster at a given location
pub fn random_monster(ecs: &mut World, x: i32, y: i32) {
    let roll: i32;
    {
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();
        roll = rng.roll_dice(1, 2);
    }
    match roll {
        1 => orc(ecs, x, y),
        _ => goblin(ecs, x, y),
    }
}

fn orc(ecs: &mut World, x: i32, y: i32) {
    monster(ecs, x, y, rltk::to_cp437('o'), "Orc");
}

fn goblin(ecs: &mut World, x: i32, y: i32) {
    monster(ecs, x, y, rltk::to_cp437('g'), "Goblin");
}

fn monster<S: ToString>(ecs: &mut World, x: i32, y: i32, glyph: rltk::FontCharType, name: S) {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph,
            fg: RGB::named(rltk::RED),
            bg: RGB::named(rltk::BLACK),
            render_order: MONSTER_ORDER,
        })
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .with(Monster {})
        .with(Name {
            name: name.to_string(),
        })
        .with(BlocksTile {})
        .with(CombatStats {
            max_hp: 16,
            hp: 16,
            defense: 1,
            power: 4,
        })
        .build();
}

fn health_potion(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: rltk::to_cp437('i'),
            fg: RGB::named(rltk::MAGENTA),
            bg: RGB::named(rltk::BLACK),
            render_order: ITEM_ORDER,
        })
        .with(Name {
            name: "Health Potion".to_string(),
        })
        .with(Item {})
        .with(Potion { heal_amount: 8 })
        .build();
}
