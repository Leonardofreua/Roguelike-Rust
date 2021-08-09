use rltk::{VirtualKeyCode, Rltk};
use specs::prelude::*;
use std::cmp::{min, max};

use super::{Position, Player, TileType, get_index_xy, State};

use crate::constants::{COORDINATE_49, COORDINATE_79};

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
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

pub fn player_input(gs: &mut State, ctx: &mut Rltk) {
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