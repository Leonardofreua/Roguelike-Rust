use rltk::{
  RGB,
  Rltk,
  RandomNumberGenerator,
  BaseMap,
  Algorithm2D,
  Point,
  SmallVec,
  DistanceAlg
};
use specs::prelude::*;
use super::{Rect};
use std::cmp::{max, min};

use crate::constants::{
  COORDINATE_79,
  WIDTH,
  HEIGHT,
  MAX_ROOMS,
  MAX_SIZE_ROOM,
  MIN_SIZE_ROOM
};

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall, Floor
}

pub struct Map {
  pub tiles: Vec<TileType>,
  pub rooms: Vec<Rect>,
  pub width: i32,
  pub height: i32,
  pub revealed_tiles: Vec<bool>,
  pub visible_tiles: Vec<bool>,
  pub blocked: Vec<bool>,
  pub tile_content : Vec<Vec<Entity>>
}

impl Map {
  pub fn get_index_xy(&self, x: i32, y: i32) -> usize {
    (y as usize * self.width as usize) + x as usize
  }

  fn apply_room_to_map(&mut self, room: &Rect) {
    for y in room.y1 + 1 ..= room.y2 {
      for x in room.x1 + 1 ..= room.x2 {
        let index = self.get_index_xy(x, y);
        self.tiles[index] = TileType::Floor;
      }
    }
  }

  fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
    for x in min(x1, x2) ..= max(x1, x2) {
      let index = self.get_index_xy(x, y);

      let map_floor_dimension: usize = (WIDTH*HEIGHT) as usize;
      if index > 0 && index < map_floor_dimension {
        self.tiles[index as usize] = TileType::Floor;
      }
    }
  }

  fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
    for y in min(y1, y2) ..= max(y1, y2) {
      let index = self.get_index_xy(x, y);
      let map_floor_dimension: usize = (WIDTH*HEIGHT) as usize;
      if index > 0 && index < map_floor_dimension {
        self.tiles[index as usize] = TileType::Floor;
      }
    }
  }

  fn is_exit_valid(&self, x:i32, y:i32) -> bool {
    if x < 1 || x > self.width - 1 || y < 1 || y > self.height - 1 { return false; }
    let index = self.get_index_xy(x, y);
    !self.blocked[index]
  }

  fn join_rooms(&mut self, rng: &mut RandomNumberGenerator, new_room: &Rect) {
    if !self.rooms.is_empty() {
      let (new_x, new_y) = new_room.center();
      let (prev_x, prev_y) = self.rooms[self.rooms.len() - 1].center();

      if rng.range(0, 2) == 1 {
        self.apply_horizontal_tunnel(prev_x, new_x, prev_y);
        self.apply_vertical_tunnel(prev_y, new_y, new_x);
      } else {
        self.apply_vertical_tunnel(prev_y, new_y, prev_x);
        self.apply_horizontal_tunnel(prev_x, new_x, new_y);
      }
    }
  }

  pub fn populate_blocked(&mut self) {
    for (index, tile) in self.tiles.iter_mut().enumerate() {
      self.blocked[index] = *tile == TileType::Wall;
    }
  }

  pub fn clear_content_index(&mut self) {
    for content in self.tile_content.iter_mut() {
      content.clear();
    }
  }

  pub fn new_rooms_and_corridors() -> Map {
    let map_floor_dimension: usize = (WIDTH*HEIGHT) as usize;
    let mut map = Map{
        tiles : vec![TileType::Wall; map_floor_dimension],
        rooms : Vec::new(),
        width : WIDTH,
        height: HEIGHT,
        revealed_tiles : vec![false; map_floor_dimension],
        visible_tiles : vec![false; map_floor_dimension],
        blocked: vec![false; map_floor_dimension],
        tile_content : vec![Vec::new(); map_floor_dimension]
    };

    let mut rng = RandomNumberGenerator::new();

    for _ in 0..MAX_ROOMS {
      let width = rng.range(MIN_SIZE_ROOM, MAX_SIZE_ROOM);
      let height = rng.range(MIN_SIZE_ROOM, MAX_SIZE_ROOM);
      let x = rng.roll_dice(1, 80 - width - 1) - 1;
      let y = rng.roll_dice(1, 50 - height - 1) - 1;
      let new_room = Rect::new(x, y, width, height);
      let mut ok = true;

      for other_room in map.rooms.iter() {
        if new_room.intersect(other_room) { ok = false }
      }

      if ok {
        map.apply_room_to_map(&new_room);
        map.join_rooms(&mut rng, &new_room);
        map.rooms.push(new_room);
      }
    }

    map
  }
}

impl BaseMap for Map {
  fn is_opaque(&self, index: usize) -> bool {
    self.tiles[index] == TileType::Wall
  }

  fn get_available_exits(&self, index: usize) -> SmallVec<[(usize, f32); 10]> {
    let mut exits = SmallVec::new();
    let x = index as i32 % self.width;
    let y = index as i32 / self.width;
    let w = self.width as usize;

    // Cardinal directions
    if self.is_exit_valid(x - 1, y) { exits.push((index - 1, 1.0)) };
    if self.is_exit_valid(x + 1, y) { exits.push((index + 1, 1.0)) };
    if self.is_exit_valid(x, y - 1) { exits.push((index - w, 1.0)) };
    if self.is_exit_valid(x, y + 1) { exits.push((index + w, 1.0)) };

    // Diagonal
    if self.is_exit_valid(x - 1, y - 1) { exits.push(((index - w) - 1, 1.45)); }
    if self.is_exit_valid(x + 1, y - 1) { exits.push(((index - w) + 1, 1.45)); }
    if self.is_exit_valid(x - 1, y + 1) { exits.push(((index + w) - 1, 1.45)); }
    if self.is_exit_valid(x + 1, y + 1) { exits.push(((index + w) + 1, 1.45)); }

    exits
  }

  fn get_pathing_distance(&self, index_1: usize, index_2: usize) -> f32 {
    let w = self.width as usize;
    let p1 = Point::new(index_1 % w, index_1 / w);
    let p2 = Point::new(index_2 % w, index_2 / w);
    DistanceAlg::Pythagoras.distance2d(p1, p2)
  }
}

impl Algorithm2D for Map {
  fn dimensions(&self) -> Point {
    Point::new(self.width, self.height)
  }
}

pub fn draw_map(ecs: &World, ctx: &mut Rltk) {
  let map = ecs.fetch::<Map>();
  let mut x = 0;
  let mut y = 0;

  for (index, tile) in map.tiles.iter().enumerate() {
    // Render a tile depending upon the tile type
    if map.revealed_tiles[index] {
      let glyph;
      let mut fg;

      match tile {
        TileType::Floor => {
            glyph = rltk::to_cp437('.');
            fg = RGB::from_f32(0.0, 0.5, 0.5);
        }
        TileType::Wall => {
            glyph = rltk::to_cp437('#');
            fg = RGB::from_f32(0., 1.0, 0.);
        }
      }

      if !map.visible_tiles[index] { fg = fg.to_greyscale() }
      ctx.set(x, y, fg, RGB::from_f32(0., 0., 0.), glyph);
    }

    // Move the coordinates
    x += 1;
    if x > COORDINATE_79 {
        x = 0;
        y += 1;
    }
  }
}