mod components;

#[path = "enums/tile_type.rs"]
mod tile_type;

use rltk::{GameState, Rltk, RGB, VirtualKeyCode};
use crate::components::{Position, Renderable, LeftMover, Player};
use std::cmp::{max, min};
use specs::prelude::*;
use tile_type::TileType;

const MAP_FLOOR_DIMENSION: usize = 80*50;
const COORDINATE_79: i32 = 79;
const COORDINATE_49: i32 = 49;

struct LeftWalker {}

struct State {
    ecs: World
}

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Vec<TileType>>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        let destination_index = get_index_xy(pos.x + delta_x, pos.y + delta_y);

        if map[destination_index] != TileType::Wall {
            pos.x = min(COORDINATE_79, max(0, pos.x + delta_x));
            pos.y = min(COORDINATE_49, max(0, pos.y + delta_y));
        }
    }
}

fn player_input(gs: &mut State, ctx: &mut Rltk) {
    // Player movement
    match ctx.key {
        None => {} // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
            _ => {}
        },
    }
}

fn get_index_xy(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

fn new_map() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; MAP_FLOOR_DIMENSION];

    for x in 0..80 {
        map[get_index_xy(x, 0)] = TileType::Wall;
        map[get_index_xy(x, COORDINATE_49)] = TileType::Wall;
    }

    for y in 0..50 {
        map[get_index_xy(0, y)] = TileType::Wall;
        map[get_index_xy(COORDINATE_79, y)] = TileType::Wall;
    }

    // Randomly splat a bunch of walls
    let mut rng = rltk::RandomNumberGenerator::new();

    for _i in 0..400 {
        let x = rng.roll_dice(1, COORDINATE_79);
        let y = rng.roll_dice(1, COORDINATE_49);
        let idx = get_index_xy(x, y);

        if idx != get_index_xy(40, 25) {
            map[idx] = TileType::Wall;
        }
    }
    map
}

// fn render_title_by_tile_type(x: i32, y: i32, tile: &TileType, ctx: &mut Rltk) {
//     match tile {
//         TileType::Floor => {
//             ctx.set(
//                 x,
//                 y, 
//                 RGB::from_f32(0.5, 0.5, 0.5), 
//                 RGB::from_f32(0., 0., 0.),
//                 rltk::to_cp437('.')
//             );
//         }
//         TileType::Wall => {
//             ctx.set(
//                 x,
//                 y,
//                 RGB::from_f32(0.0, 1.0, 0.0),
//                 RGB::from_f32(0., 0., 0.),
//                 rltk::to_cp437('#')
//             );
//         }
//     }
// }

fn set_tile_wall(x: i32, y: i32, ctx: &mut Rltk) {
    ctx.set(
        x,
        y, 
        RGB::from_f32(0.5, 0.5, 0.5), 
        RGB::from_f32(0., 0., 0.),
        rltk::to_cp437('.')
    );
}

fn set_tile_floor(x: i32, y: i32, ctx: &mut Rltk) {
    ctx.set(
        x,
        y,
        RGB::from_f32(0.0, 1.0, 0.0),
        RGB::from_f32(0., 0., 0.),
        rltk::to_cp437('#')
    );
}

fn draw_map(map: &[TileType], ctx: &mut Rltk) {
    let mut x = 0;
    let mut y = 0;

    for tile in map.iter() {
        // Render a tile depending upon the tile type
        match tile {
            TileType::Floor => { set_tile_wall(x, y, ctx); }
            TileType::Wall => { set_tile_floor(x, y, ctx); }
        }

        // Move the coordinates
        x += 1;
        if x > COORDINATE_79 {
            x = 0;
            y += 1;
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();
        
        player_input(self, ctx);
        self.run_systems();

        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        
        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

impl State {
    fn run_systems(&mut self) {
        let mut left_walker = LeftWalker{};
        left_walker.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl<'a> System<'a> for LeftWalker {
    type SystemData = (
        ReadStorage<'a, LeftMover>,
        WriteStorage<'a, Position>
    );

    fn run(&mut self, (lefty, mut pos) : Self::SystemData) {
        for (_lefty, pos) in (&lefty, &mut pos).join() {
            pos.x -= 1;
            if pos.x < 0 { pos.x = COORDINATE_79; }
        }
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;

    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Game")
        .build()?;
    let mut gs = State {
        ecs: World::new()
    };

    // Registering the components
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<LeftMover>();
    gs.ecs.register::<Player>();
    
    // Set a new map
    gs.ecs.insert(new_map());

    // Creating Player entity
    gs.ecs
        .create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player{})
        .build();

    rltk::main_loop(context, gs)
}
