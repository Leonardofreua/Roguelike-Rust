use specs::prelude::*;
use rltk::{GameState, Rltk, RGB};

mod components;
mod constants;
mod player;
mod map;
mod rect;
mod visibility_system;

pub use constants::{COORDINATE_79, VISIBLE_TILES_RANGE};
pub use components::{Position, Renderable, LeftMover, Player, Viewshed};
pub use map::{Map, TileType, draw_map};
pub use rect::Rect;
use player::player_input;
use visibility_system::VisibilitySystem;

pub struct State {
    ecs: World
}

impl State {
    fn run_systems(&mut self) {
        let mut visibility = VisibilitySystem{};
        visibility.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();
        
        player_input(self, ctx);
        self.run_systems();

        draw_map(&self.ecs, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        
        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
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
    gs.ecs.register::<Viewshed>();
    
    // Set a new map
    let map: Map = Map::new_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();
    gs.ecs.insert(map);

    // Creating Player entity
    gs.ecs
        .create_entity()
        .with(Position { x: player_x, y: player_y })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player{})
        .with(Viewshed{ visible_tiles: Vec::new(), range: VISIBLE_TILES_RANGE, dirty: true })
        .build();

    rltk::main_loop(context, gs)
}
