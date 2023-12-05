//! A map consisting of tiles.
mod array;
mod chunk;
mod generate;
mod render;
mod neighbors;
mod edit;
mod debug;

use self::chunk::{ChunkId, ChunkTiles, Chunks};
use self::edit::EditorConf;
use self::generate::GenerateConf;
use self::render::ChunkMaterial;
use crate::camera::GameCamera;
use bevy::input::common_conditions::input_pressed;
use bevy::prelude::*;
use bevy::sprite::{Material2dPlugin, MaterialMesh2dBundle};
use bevy::utils::HashSet;

/// The sqrt of the number of tiles within a chunk.
pub const CHUNK_TILES_SQ: usize = 50;
/// The total number of tiles within a chunk.
pub const CHUNK_TILES: usize = CHUNK_TILES_SQ * CHUNK_TILES_SQ;
/// The sqrt of the size of a chunk in pixels.
pub const CHUNK_SIZE_SQ: f32 = CHUNK_TILES_SQ as f32 * 8.0;

pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(Chunks::default())
      .insert_resource(EditorConf::default())
      .insert_resource(GenerateConf {
        seed: 0xdead,
        continent_scale: 1.0
      })
      .add_plugins(Material2dPlugin::<render::ChunkMaterial>::default())
      .add_systems(Update, (
        chunk::spawn, 
        chunk::cleanup,
        update_precense, 
        render::update_material,
        edit::draw.run_if(input_pressed(MouseButton::Left)),
        debug::chunk_wireframes,
        edit::menu,
        // ..
      ))
    // ..
    ;
  }
}

#[allow(clippy::type_complexity)]
#[tracing::instrument(skip_all)]
fn update_precense(
  mut chunks: ResMut<Chunks>,
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ChunkMaterial>>,
  assets: Res<AssetServer>,
  cameras: Query<
    (&Transform, &OrthographicProjection),
    (
      With<GameCamera>,
      Or<(Changed<Transform>, Changed<OrthographicProjection>)>,
    ),
  >,
) {
  let plane = shape::Quad::new(Vec2::splat(CHUNK_SIZE_SQ));
  let mesh = meshes.add(Mesh::from(plane));

  for (transform, projection) in &cameras {
    let mut in_view = HashSet::new();

    let view_min = projection.area.min + transform.translation.xy();
    let view_max = projection.area.max + transform.translation.xy();

    let min = ChunkId::from_world(view_min);
    let max = ChunkId::from_world(view_max);

    let dist = 4;

    for x in min.x-dist..max.x +dist{
      for y in min.y-dist..max.y+dist {
        let id = ChunkId(IVec2::new(x, y));

        _ = tracing::debug_span!("spawn", id = ?id).entered();
        in_view.insert(id);

        if chunks.contains_key(&id) {
          continue;
        }

        let entity = commands.spawn((
          id,
          ChunkTiles::default(),
          MaterialMesh2dBundle {
            mesh: mesh.clone().into(),
            material: materials.add(ChunkMaterial::from_image(assets.load("sprites/tiles.png"))),
            transform: Transform::from_xyz(x as f32 * plane.size.x, y as f32 * plane.size.y, 0.0),
            ..Default::default()
          },
        ));

        chunks.insert(id, entity.id());
      }
    }

    for (id, entity) in chunks.clone() {
      if in_view.contains(&id) {
        continue;
      }

      chunks.remove(&id);
      commands.entity(entity).despawn_recursive();
    }
  }
}

