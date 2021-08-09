use rltk::{RGB, Rltk, RandomNumberGenerator};
use std::cmp::{max, min};

use crate::constants::{COORDINATE_49, COORDINATE_79, MAP_FLOOR_DIMENSION};

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall, Floor
}

pub fn get_index_xy(x: i32, y: i32) -> usize {
  (y as usize * 80) + x as usize
}

pub fn new_map() -> Vec<TileType> {
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