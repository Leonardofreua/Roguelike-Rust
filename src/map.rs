use rltk::{ RGB, Rltk, RandomNumberGenerator, BaseMap, Algorithm2D, Point };
use specs::prelude::{World};
use super::{Rect};
use std::cmp::{max, min};

use crate::constants::{
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

pub struct Map {
  pub tiles: Vec<TileType>,
  pub rooms: Vec<Rect>,
  pub width: i32,
  pub height: i32,
  pub revealed_tiles: Vec<bool>,
  pub visible_tiles: Vec<bool>
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
      if index > 0 && index < MAP_FLOOR_DIMENSION {
        self.tiles[index as usize] = TileType::Floor;
      }
    }
  }
  
  fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
    for y in min(y1, y2) ..= max(y1, y2) {
      let index = self.get_index_xy(x, y);
      if index > 0 && index < MAP_FLOOR_DIMENSION {
        self.tiles[index as usize] = TileType::Floor;
      }
    }
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

  pub fn new_map_rooms_and_corridors() -> Map {
    let mut map = Map{
        tiles : vec![TileType::Wall; 80*50],
        rooms : Vec::new(),
        width : 80,
        height: 50,
        revealed_tiles : vec![false; 80*50],
        visible_tiles : vec![false; 80*50]
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