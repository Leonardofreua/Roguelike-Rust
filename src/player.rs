use rltk::{VirtualKeyCode, Rltk, Point};
use specs::prelude::*;
use std::cmp::{min, max};

use super::{Position, Player, State, Map, Viewshed, RunState};

use crate::constants::{COORDINATE_49, COORDINATE_79};

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
  let mut positions = ecs.write_storage::<Position>();
  let mut players = ecs.write_storage::<Player>();
  let mut viewsheds = ecs.write_storage::<Viewshed>();
  let map = ecs.fetch::<Map>();

  for (_player, pos, viewshed) in (&mut players, &mut positions, &mut viewsheds).join() {
      let destination_index = map.get_index_xy(pos.x + delta_x, pos.y + delta_y);

      if !map.blocked[destination_index] {
          pos.x = min(COORDINATE_79, max(0, pos.x + delta_x));
          pos.y = min(COORDINATE_49, max(0, pos.y + delta_y));

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
      None => { return RunState::Paused }
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

          _ => { return RunState::Paused }
      },
  }
  RunState::Running
}