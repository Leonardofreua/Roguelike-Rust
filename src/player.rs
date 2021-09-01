use specs::prelude::*;
use rltk::{VirtualKeyCode, Rltk, Point};
use std::cmp::{min, max};

use super::{
  Position,
  Player,
  State,
  Map,
  Viewshed,
  RunState,
  CombatStats,
  WantsToMelee
};

use crate::constants::{COORDINATE_Y, COORDINATE_X};

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
  let mut positions = ecs.write_storage::<Position>();
  let mut viewsheds = ecs.write_storage::<Viewshed>();
  let mut wants_to_melee = ecs.write_storage::<WantsToMelee>();
  let players = ecs.write_storage::<Player>();
  let combat_stats = ecs.write_storage::<CombatStats>();
  let entities = ecs.entities();
  let map = ecs.fetch::<Map>();

  for (entity, _player, pos, viewshed) in (&entities, & players, &mut positions, &mut viewsheds).join() {
    let sum_x_coordinates = pos.x + delta_x;
    let sum_y_coordinates = pos.y + delta_y;
    
    if sum_x_coordinates < 1 || sum_x_coordinates > map.width - 1 || sum_y_coordinates < 1 || sum_y_coordinates > map.height - 1 { return; }
    
    let destination_index = map.get_index_xy(sum_x_coordinates, sum_y_coordinates);
    for potential_target in map.tile_content[destination_index].iter() {
      let target = combat_stats.get(*potential_target);
      if let Some(_target) = target {
        wants_to_melee
          .insert(entity, WantsToMelee { target: *potential_target })
          .expect("Add target failed");
      }
    }

    if !map.blocked[destination_index] {
        pos.x = min(COORDINATE_X, max(0, sum_x_coordinates));
        pos.y = min(COORDINATE_Y, max(0, sum_y_coordinates));

        viewshed.dirty = true;
        let mut ppos = ecs.write_resource::<Point>();
        ppos.x = pos.x;
        ppos.y = pos.y;
    }
  }
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) -> RunState {
  // Player movement
  match ctx.key {
      None => { return RunState::AwaitingInput }
      Some(key) => match key {
          VirtualKeyCode::Left |
          VirtualKeyCode::Numpad4 |
          VirtualKeyCode::H |
          VirtualKeyCode::A => try_move_player(-1, 0, &mut gs.ecs),

          VirtualKeyCode::Right |
          VirtualKeyCode::Numpad6 |
          VirtualKeyCode::L |
          VirtualKeyCode::D => try_move_player(1, 0, &mut gs.ecs),

          VirtualKeyCode::Up |
          VirtualKeyCode::Numpad8 |
          VirtualKeyCode::K |
          VirtualKeyCode::W => try_move_player(0, -1, &mut gs.ecs),

          VirtualKeyCode::Down |
          VirtualKeyCode::Numpad2 |
          VirtualKeyCode::J |
          VirtualKeyCode::S => try_move_player(0, 1, &mut gs.ecs),

          // Diagonals
          VirtualKeyCode::Numpad9 |
          VirtualKeyCode::Y => try_move_player(1, -1, &mut gs.ecs),

          VirtualKeyCode::Numpad7 |
          VirtualKeyCode::U => try_move_player(-1, -1, &mut gs.ecs),

          VirtualKeyCode::Numpad3 |
          VirtualKeyCode::N => try_move_player(1, 1, &mut gs.ecs),

          VirtualKeyCode::Numpad1 |
          VirtualKeyCode::B => try_move_player(-1, 1, &mut gs.ecs),

          _ => { return RunState::AwaitingInput }
      },
  }
  RunState::PlayerTurn
}