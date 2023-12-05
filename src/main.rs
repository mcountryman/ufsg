use bevy::prelude::*;

mod camera;
mod debug;
mod sprites;
mod tilemap;

fn main() {
  App::new()
    // .insert_resource(ClearColor(Color::BLUE))
    .add_plugins(debug::DebugPlugin)
    //
    .insert_resource(Msaa::Off)
    .add_plugins(
      DefaultPlugins
        .set(WindowPlugin {
          primary_window: Some(Window {
            title: "untitled RTS gaem that'll totally be finished and not be another dead project in 2 weeks".into(),
            ..Default::default()
          }),
          ..Default::default()
        })
        .set(ImagePlugin::default_nearest()),
    )
    .add_plugins(bevy_egui::EguiPlugin)
    // .add_plugins(bevy::diagnostic::LogDiagnosticsPlugin::default())
    .add_plugins(camera::CameraPlugin)
    .add_plugins(tilemap::TileMapPlugin)
    .run();
}
