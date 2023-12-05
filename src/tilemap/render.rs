use super::chunk::ChunkTiles;
use super::CHUNK_TILES;
use crate::sprites::Tiles;
use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::Material2d;

/// A material that renders a chunk of tiles.
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct ChunkMaterial {
  #[texture(1)]
  #[sampler(2)]
  image: Handle<Image>,
  #[storage(3)]
  tiles: [u32; CHUNK_TILES],
  #[uniform(4)]
  tile_size: u32,
}

impl ChunkMaterial {
  /// Creates a [ChunkMaterial] from the given image.
  pub fn from_image(image: Handle<Image>) -> Self {
    Self {
      image,
      tiles: [Tiles::Void as _; CHUNK_TILES],
      tile_size: 8,
    }
  }
}

impl Material2d for ChunkMaterial {
  fn fragment_shader() -> ShaderRef {
    "shaders/tiles.wgsl".into()
  }
}

/// Updates [ChunkMaterial] using associated [ChunkTiles].
#[tracing::instrument(skip_all)]
pub fn update_material(
  query: Query<(&Handle<ChunkMaterial>, &ChunkTiles), Changed<ChunkTiles>>,
  mut materials: ResMut<Assets<ChunkMaterial>>,
) {
  for (handle, tiles) in &query {
    _ = tracing::debug_span!("chunk").entered();

    let material = materials.get_mut(handle).unwrap();

    for (from, to) in tiles.iter().zip(material.tiles.iter_mut()) {
      *to = *from as _;
    }
  }
}
