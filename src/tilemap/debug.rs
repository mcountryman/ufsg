use super::chunk::Chunks;
use crate::camera::GameCamera;
use bevy::prelude::*;

pub fn chunk_wireframes(
  mut gizmos: Gizmos,
  chunks: Res<Chunks>,
  camera: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
  windows: Query<&Window>,
) {
  for chunk in chunks.keys() {
    gizmos.rect_2d(
      chunk.to_world_rect().center(),
      0.0,
      chunk.to_world_rect().size(),
      Color::BLUE,
    );
  }

  let (camera, camera_transform) = camera.single();
  let cursors = windows
    .iter()
    .filter_map(|window| window.cursor_position())
    .filter_map(|coord| camera.viewport_to_world_2d(camera_transform, coord));

  for pos in cursors {
    for chunk in chunks.keys() {
      if chunk.to_world_rect().contains(pos) {
        gizmos.rect_2d(
          chunk.to_world_rect().center(),
          0.0,
          chunk.to_world_rect().size(),
          Color::RED,
        );
      }
    }
  }
}
