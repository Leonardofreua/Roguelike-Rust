use specs::prelude::*;
use rltk::{GameState, Rltk, RGB, Point};

mod components;
mod constants;
mod player;
mod map;
mod rect;
mod visibility_system;
mod monster_ai_system;

pub use constants::{COORDINATE_79, VISIBLE_TILES_RANGE};
pub use components::{Position, Renderable, LeftMover, Player, Viewshed, Monster, Name};
pub use map::{Map, TileType, draw_map};
pub use rect::Rect;
use player::player_input;
use visibility_system::VisibilitySystem;
use monster_ai_system::MonsterAI;

#[derive(PartialEq, Copy, Clone)]
pub enum RunState { Paused, Running }

pub struct State {
    pub ecs: World,
    pub runstate: RunState,
}

impl State {
    fn run_systems(&mut self) {
        let mut visibility = VisibilitySystem{};
        visibility.run_now(&self.ecs);

        let mut mob = MonsterAI{};
        mob.run_now(&self.ecs);

        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();

        if self.runstate == RunState::Running {
            self.run_systems();
            self.runstate = RunState::Paused;
        } else {
            self.runstate = player_input(self, ctx);
        }

        draw_map(&self.ecs, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Map>();

        for (pos, render) in (&positions, &renderables).join() {
            let index = map.get_index_xy(pos.x, pos.y);
            if map.visible_tiles[index] {
                ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
            }
        }
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;

    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Game")
        .build()?;
    let mut gs = State {
        ecs: World::new(),
        runstate: RunState::Running,
    };

    // Registering the components
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<LeftMover>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Monster>();
    gs.ecs.register::<Name>();

    // Set a new map
    let map: Map = Map::new_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();

    let mut rng = rltk::RandomNumberGenerator::new();
    for (i, room) in map.rooms.iter().skip(1).enumerate() {
        let (x, y) = room.center();

        let glyph: rltk::FontCharType;
        let name: String;
        let roll = rng.roll_dice(1, 2);

        match roll {
            1 => { glyph = rltk::to_cp437('g'); name = "Goblin".to_owned(); }
            _ => { glyph = rltk::to_cp437('o'); name = "Orc".to_owned(); }
        }

        gs.ecs.create_entity()
            .with(Position { x, y })
            .with(Renderable{
                glyph: glyph,
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .with(Viewshed{ visible_tiles: Vec::new(), range: VISIBLE_TILES_RANGE, dirty: true })
            .with(Monster{})
            .with(Name{ name: format!("{} #{}", &name, i) })
            .build();
    }

    gs.ecs.insert(map);
    gs.ecs.insert(Point::new(player_x, player_y));

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
        .with(Name{ name: "Player".to_owned() })
        .build();

    rltk::main_loop(context, gs)
}
