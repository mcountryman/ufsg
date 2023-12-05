use super::chunk::ChunkTiles;
use crate::sprites::Tiles;
use bevy::math::{IVec2, UVec2};
use std::marker::PhantomData;
use std::ptr::NonNull;

#[derive(Copy, Clone, PartialEq)]
pub enum NeighborDirection {
  North,
  East,
  West,
  South,
}

impl NeighborDirection {
  pub fn offset(&self) -> IVec2 {
    match self {
      Self::North => IVec2::new(0, 1),
      Self::East => IVec2::new(-1, 0),
      Self::West => IVec2::new(1, 0),
      Self::South => IVec2::new(0, -1),
    }
  }

  pub fn offset_of(&self, of: UVec2) -> Option<UVec2> {
    Some(match self {
      Self::North => UVec2::new(of.x, of.y.checked_add(1)?),
      Self::East => UVec2::new(of.x.checked_sub(1)?, of.y),
      Self::West => UVec2::new(of.x.checked_add(1)?, of.y),
      Self::South => UVec2::new(of.x, of.y.checked_sub(1)?),
    })
  }
}

impl NeighborDirection {
  pub const CLOCKWISE: [Self; 4] = [Self::North, Self::West, Self::South, Self::East];
}

#[derive(Clone, Copy)]
pub struct Neighbors<T> {
  pub north: Option<T>,
  pub east: Option<T>,
  pub west: Option<T>,
  pub south: Option<T>,
}

impl<T> Neighbors<T> {
  pub fn get(&self, dir: NeighborDirection) -> Option<&T> {
    match dir {
      NeighborDirection::North => self.north.as_ref(),
      NeighborDirection::East => self.east.as_ref(),
      NeighborDirection::West => self.west.as_ref(),
      NeighborDirection::South => self.south.as_ref(),
    }
  }

  pub fn get_mut(&mut self, dir: NeighborDirection) -> &mut Option<T> {
    match dir {
      NeighborDirection::North => &mut self.north,
      NeighborDirection::East => &mut self.east,
      NeighborDirection::West => &mut self.west,
      NeighborDirection::South => &mut self.south,
    }
  }

  pub fn map<U, F>(self, f: F) -> Neighbors<U>
  where
    F: FnOnce(T) -> U + Copy,
  {
    Neighbors {
      north: self.north.map(f),
      east: self.east.map(f),
      west: self.west.map(f),
      south: self.south.map(f),
    }
  }

  pub fn filter<F>(self, f: F) -> Neighbors<T>
  where
    F: FnOnce(&T) -> bool + Copy,
  {
    Neighbors {
      north: self.north.filter(f),
      east: self.east.filter(f),
      west: self.west.filter(f),
      south: self.south.filter(f),
    }
  }

  pub fn len(&self) -> usize {
    self.iter().count()
  }

  pub fn is_empty(&self) -> bool {
    self.len() == 0
  }

  pub fn iter(&self) -> Iter<'_, T> {
    Iter {
      dir: 0,
      neighbors: self,
    }
  }

  pub fn iter_mut(&mut self) -> IterMut<'_, T> {
    IterMut {
      dir: 0,
      phantom: Default::default(),
      neighbors: self.into(),
    }
  }

  pub fn into_flat_array(self) -> Option<[T; 4]> {
    Some([self.north?, self.west?, self.south?, self.east?])
  }
}

impl Neighbors<Tiles> {
  pub fn from_chunk_tiles(tile_id: UVec2, chunk_tiles: &ChunkTiles) -> Self {
    let mut neighbors = Self::default();

    for dir in NeighborDirection::CLOCKWISE {
      if let Some(id) = dir.offset_of(tile_id) {
        if let Some(tile) = chunk_tiles.get(id) {
          *neighbors.get_mut(dir) = Some(*tile);
        }
      }
    }

    neighbors
  }
}

impl<T> Default for Neighbors<T> {
  fn default() -> Self {
    Self {
      north: None,
      east: None,
      west: None,
      south: None,
    }
  }
}

impl<'a, T> IntoIterator for &'a Neighbors<T> {
  type Item = &'a T;
  type IntoIter = Iter<'a, T>;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

impl<'a, T> IntoIterator for &'a mut Neighbors<T> {
  type Item = &'a mut T;
  type IntoIter = IterMut<'a, T>;

  fn into_iter(self) -> Self::IntoIter {
    self.iter_mut()
  }
}

pub struct Iter<'a, T> {
  dir: usize,
  neighbors: &'a Neighbors<T>,
}

impl<'a, T: 'a> Iterator for Iter<'a, T> {
  type Item = &'a T;

  fn next(&mut self) -> Option<Self::Item> {
    loop {
      let dir = NeighborDirection::CLOCKWISE.get(self.dir)?;
      let val = self.neighbors.get(*dir);

      self.dir += 1;

      if val.is_some() {
        return val;
      }
    }
  }
}

pub struct IterMut<'a, T> {
  dir: usize,
  phantom: PhantomData<&'a ()>,
  neighbors: NonNull<Neighbors<T>>,
}

impl<'a, T: 'a> Iterator for IterMut<'a, T> {
  type Item = &'a mut T;

  fn next(&mut self) -> Option<Self::Item> {
    loop {
      let dir = NeighborDirection::CLOCKWISE.get(self.dir)?;
      let val = unsafe { self.neighbors.as_mut() };
      let val = val.get_mut(*dir);

      self.dir += 1;

      if val.is_some() {
        return val.as_mut();
      }
    }
  }
}
