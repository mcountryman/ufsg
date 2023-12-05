use super::array::TileArray;
use super::generate::{self, GenerateConf};
use super::neighbors::Neighbors;
use super::CHUNK_SIZE_SQ;
use crate::sprites::Tiles;
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Default, Deref, DerefMut, Resource)]
pub struct Chunks(HashMap<ChunkId, Entity, fxhash::FxBuildHasher>);

#[derive(Debug, Clone, Copy, Deref, DerefMut, PartialEq, Eq, Hash, Component)]
pub struct ChunkId(pub IVec2);

impl ChunkId {
  pub fn from_world(coord: Vec2) -> Self {
    Self((coord / CHUNK_SIZE_SQ).as_ivec2())
  }

  pub fn to_world(self) -> Vec2 {
    (self.0.as_vec2() * CHUNK_SIZE_SQ) - (CHUNK_SIZE_SQ / 2.0)
  }

  pub fn to_world_rect(self) -> Rect {
    let min = self.to_world();
    let max = min + CHUNK_SIZE_SQ;

    Rect::from_corners(min, max)
  }
}

impl From<IVec2> for ChunkId {
  fn from(value: IVec2) -> Self {
    Self(value)
  }
}

#[derive(Deref, DerefMut, Component)]
pub struct ChunkTiles(TileArray<Tiles>);

impl Default for ChunkTiles {
  fn default() -> Self {
    Self(TileArray::of(Tiles::Void))
  }
}

#[tracing::instrument(skip_all)]
pub fn spawn(
  conf: Res<GenerateConf>,
  mut chunks: Query<(&ChunkId, &mut ChunkTiles), Added<ChunkId>>,
) {
  for (id, mut tiles) in &mut chunks {
    _ = tracing::debug_span!("chunk", id = ?id).entered();

    generate::generate(&conf, *id, &mut tiles);
  }
}

pub fn cleanup(mut _tiles: Query<&mut ChunkTiles, Changed<ChunkTiles>>) {
  // for mut tiles in &mut tiles {
  //   for _ in 0..3 {
  //     for x in 0..CHUNK_TILES_SQ as _ {
  //       for y in 0..CHUNK_TILES_SQ as _ {
  //         let id = UVec2::new(x, y);
  //         let neighbors = Neighbors::from_chunk_tiles(id, &tiles);

  //         match tiles.get_mut(id) {
  //           Some(tile) if tile.is_sand() => cleanup_sand(tile, &neighbors),
  //           Some(tile) if tile.is_grass() => cleanup_grass(tile, &neighbors),
  //           Some(tile) if tile.is_water() => cleanup_water(tile, &neighbors),
  //           _ => {}
  //         };
  //       }
  //     }
  //   }
  // }
}

fn cleanup_water(tile: &mut Tiles, neighbors: &Neighbors<Tiles>) {
  let land = neighbors.map(|t| t.is_land()).filter(|t| *t);

  if !land.is_empty() && tile.is_water_deep() {
    *tile = Tiles::WaterShallow;
  }
}

fn cleanup_sand(tile: &mut Tiles, neighbors: &Neighbors<Tiles>) {
  let water = neighbors.map(|t| t.is_water()).filter(|t| *t);
  if water.is_empty() {
    return;
  }

  if water.len() == 4 {
    *tile = Tiles::WaterShallow;
    return;
  }

  if water.north == Some(true) {
    if water.east == Some(true) {
      *tile = Tiles::BeachTopRight;
    } else if water.west == Some(true) {
      *tile = Tiles::BeachTopLeft;
    } else {
      *tile = Tiles::BeachTop;
    }
  } else if water.south == Some(true) {
    if water.east == Some(true) {
      *tile = Tiles::BeachBottomLeft;
    } else if water.west == Some(true) {
      *tile = Tiles::BeachBottomRight;
    } else {
      *tile = Tiles::BeachBottom;
    }
  } else if water.east == Some(true) {
    *tile = Tiles::BeachLeft;
  } else if water.west == Some(true) {
    *tile = Tiles::BeachRight;
  }
}

fn cleanup_grass(tile: &mut Tiles, neighbors: &Neighbors<Tiles>) {
  let water = neighbors.map(|t| t.is_water()).filter(|t| *t);
  if water.len() == 4 {
    *tile = Tiles::WaterShallow;
  }
}
