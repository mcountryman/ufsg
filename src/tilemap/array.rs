use super::{CHUNK_TILES, CHUNK_TILES_SQ};
use bevy::prelude::*;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, Deref, DerefMut)]
pub struct TileArray<T>([T; CHUNK_TILES]);

impl<T> TileArray<T> {
  pub fn get(&self, pos: UVec2) -> Option<&T> {
    self.0.get(index(pos))
  }

  pub fn get_mut(&mut self, pos: UVec2) -> Option<&mut T> {
    self.0.get_mut(index(pos))
  }

  pub fn set(&mut self, pos: UVec2, tile: T) {
    if let Some(x) = self.get_mut(pos) {
      *x = tile;
    }
  }
}

impl<T: Copy> TileArray<T> {
  pub fn of(tile: T) -> Self {
    Self([tile; CHUNK_TILES])
  }
}

impl<T: Copy + Default> Default for TileArray<T> {
  fn default() -> Self {
    Self::of(T::default())
  }
}

fn index(pos: UVec2) -> usize {
  pos.x as usize * CHUNK_TILES_SQ + pos.y as usize
}
