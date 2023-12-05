use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::input::touchpad::TouchpadMagnify;
use bevy::prelude::*;
use bevy_egui::EguiContexts;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, startup)
      .add_systems(Update, (pan, zoom));
  }
}

#[derive(Component)]
pub struct GameCamera;

fn startup(mut commands: Commands) {
  commands.spawn((GameCamera, {
    let mut camera = Camera2dBundle::default();

    camera.projection.scale = 0.55;
    camera
  }));
}

fn pan(
  time: Res<Time>,
  mut egui: EguiContexts,
  mut input: EventReader<MouseWheel>,
  mut camera: Query<&mut Transform, With<GameCamera>>,
) {
  if egui.ctx_mut().is_pointer_over_area() {
    return;
  }

  let mpps = 180.0;
  let time_factor = 100.0;

  for event in input.read() {
    let mut translation = match event.unit {
      MouseScrollUnit::Line => Vec2::new(event.x * 18.0, event.y * 18.0),
      MouseScrollUnit::Pixel => Vec2::new(event.x, event.y),
    };

    translation *= time.delta_seconds() * time_factor;
    translation *= Vec2::new(-1.0, 1.0);
    translation = translation.clamp(Vec2::splat(-mpps), Vec2::splat(mpps));

    for mut transform in &mut camera {
      transform.translation += translation.extend(0.0);
    }
  }
}

fn zoom(
  mut egui: EguiContexts,
  mut input: EventReader<TouchpadMagnify>,
  mut camera: Query<&mut OrthographicProjection, With<GameCamera>>,
) {
  if egui.ctx_mut().is_pointer_over_area() {
    return;
  }

  for TouchpadMagnify(by) in input.read() {
    for mut projection in camera.iter_mut() {
      projection.scale += -by;
      projection.scale = 0.1f32.max(projection.scale);
      projection.scale = 10.0f32.min(projection.scale);
    }
  }
}
