use bevy::prelude::*;

pub fn setup_camera(mut commands: Commands) {
    commands.insert_resource(ClearColor(Color::rgb(1., 1., 1.)));
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}
