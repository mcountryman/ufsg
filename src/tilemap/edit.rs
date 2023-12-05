use super::chunk::{ChunkTiles, Chunks};
use super::CHUNK_TILES_SQ;
use crate::camera::GameCamera;
use crate::sprites::Tiles;
use bevy::prelude::*;
use bevy_egui::egui::Align2;
use bevy_egui::{egui, EguiContexts};

#[derive(Resource)]
pub struct EditorConf {
  brush_size: f32,
  brush_tile: Tiles,
}

impl Default for EditorConf {
  fn default() -> Self {
    Self {
      brush_size: 5.0,
      brush_tile: Tiles::Void,
    }
  }
}

pub fn menu(mut conf: ResMut<EditorConf>, mut egui: EguiContexts) {
  egui::Window::new("editor")
    .anchor(Align2::RIGHT_TOP, (-10.0, 10.0))
    .title_bar(false)
    .resizable(false)
    .show(egui.ctx_mut(), |ui| {
      egui::Grid::new("controls")
        .num_columns(2)
        .striped(true)
        .show(ui, |ui| {
          ui.label("Brush size");
          ui.add(egui::Slider::new(&mut conf.brush_size, 1.0..=50.0));
          ui.end_row();

          ui.label("Brush tile");
          egui::ComboBox::from_label("")
            .selected_text(format!("{:?}", conf.brush_tile))
            .show_ui(ui, |ui| {
              ui.set_min_width(200.0);

              for tile in Tiles::ALL {
                ui.selectable_value(&mut conf.brush_tile, tile, format!("{:?}", tile));
              }
            });
          ui.end_row();
        })
    });
}

pub fn draw(
  conf: Res<EditorConf>,
  chunks: Res<Chunks>,
  camera: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
  windows: Query<&Window>,
  mut egui: EguiContexts,
  mut tiles: Query<&mut ChunkTiles>,
) {
  if egui.ctx_mut().is_pointer_over_area() {
    return;
  }

  let (camera, camera_transform) = camera.single();
  let cursors = windows
    .iter()
    .filter_map(|window| window.cursor_position())
    .filter_map(|coord| camera.viewport_to_world_2d(camera_transform, coord));

  let brush_size = (conf.brush_size - 0.5) * 8.0f32;
  let brush_size = brush_size.powf(2.0);

  for cursor_pos in cursors {
    let brush_rect = Rect::from_corners(cursor_pos - brush_size, cursor_pos + brush_size);

    for (chunk_id, chunk) in chunks.iter() {
      let chunk_rect = chunk_id.to_world_rect();
      if chunk_rect.intersect(brush_rect).is_empty() {
        continue;
      }

      let Ok(mut chunk) = tiles.get_mut(*chunk) else {
        continue;
      };

      for x in 0..CHUNK_TILES_SQ as _ {
        for y in 0..CHUNK_TILES_SQ as _ {
          let tile_id = UVec2::new(x, y);
          let tile_pos = UVec2::new(x, y).as_vec2() * 8.0 + chunk_id.to_world();
          let tile_pos = tile_pos + 4.0;

          let distance = cursor_pos.distance_squared(tile_pos);
          if distance > brush_size {
            continue;
          }

          chunk.set(tile_id, conf.brush_tile);
        }
      }
    }
  }
}
