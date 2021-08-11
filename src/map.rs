use rltk::{RGB, Rltk, RandomNumberGenerator};
use super::{Rect};
use std::cmp::{max, min};

use crate::constants::{
  COORDINATE_49,
  COORDINATE_79,
  MAP_FLOOR_DIMENSION,
  MAX_ROOMS,
  MAX_SIZE_ROOM,
  MIN_SIZE_ROOM
};

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall, Floor
}

pub fn get_index_xy(x: i32, y: i32) -> usize {
  (y as usize * 80) + x as usize
}

/// Make a map with solid boundaries and 400 randomly placed walls
pub fn new_map_test() -> Vec<TileType> {
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
  let mut rng = RandomNumberGenerator::new();

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

fn apply_room_to_map(room: &Rect, map: &mut [TileType]) {
  for y in room.y1 + 1 ..= room.y2 {
    for x in room.x1 + 1 ..= room.x2 {
      map[get_index_xy(x, y)] = TileType::Floor;
    }
  }
}

fn join_rooms(
  map: &mut [TileType],
  rng: &mut RandomNumberGenerator,
  rooms: &Vec<Rect>,
  new_room: &Rect) {

  if !rooms.is_empty() {
    let (new_x, new_y) = new_room.center();
    let (prev_x, prev_y) = rooms[rooms.len() - 1].center();

    if rng.range(0, 2) == 1 {
      apply_horizontal_tunnel(map, prev_x, new_x, prev_y);
      apply_vertical_tunnel(map, prev_y, new_y, new_x);
    } else {
      apply_vertical_tunnel(map, prev_y, new_y, prev_x);
      apply_horizontal_tunnel(map, prev_x, new_x, new_y);
    }
  }
}

pub fn new_map_rooms_and_corridors() -> (Vec<Rect>, Vec<TileType>) {
  let mut map = vec![TileType::Wall; MAP_FLOOR_DIMENSION];
  let mut rooms: Vec<Rect> = Vec::new();
  let mut rng = RandomNumberGenerator::new();

  for _ in 0..MAX_ROOMS {
    let width = rng.range(MIN_SIZE_ROOM, MAX_SIZE_ROOM);
    let height = rng.range(MIN_SIZE_ROOM, MAX_SIZE_ROOM);
    let x = rng.roll_dice(1, 80 - width - 1) - 1;
    let y = rng.roll_dice(1, 50 - height - 1) - 1;
    let new_room = Rect::new(x, y, width, height);
    let mut ok = true;

    for other_room in rooms.iter() {
      if new_room.intersect(other_room) { ok = false }
    }

    if ok {
      apply_room_to_map(&new_room, &mut map);
      join_rooms(&mut map, &mut rng, &rooms, &new_room);
      rooms.push(new_room);
    }
  }

  (rooms, map)
}

fn apply_horizontal_tunnel(map: &mut [TileType], x1: i32, x2: i32, y: i32) {
  for x in min(x1, x2) ..= max(x1, x2) {
    let index = get_index_xy(x, y);
    if index > 0 && index < MAP_FLOOR_DIMENSION {
      map[index as usize] = TileType::Floor;
    }
  }
}

fn apply_vertical_tunnel(map: &mut [TileType], y1: i32, y2: i32, x: i32) {
  for y in min(y1, y2) ..= max(y1, y2) {
    let index = get_index_xy(x, y);
    if index > 0 && index < MAP_FLOOR_DIMENSION {
      map[index as usize] = TileType::Floor;
    }
  }
}

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

pub fn draw_map(map: &[TileType], ctx: &mut Rltk) {
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