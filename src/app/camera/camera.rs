use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

pub fn setup(mut commands: Commands) {
    commands.insert_resource(ClearColor(Color::rgb(1., 1., 1.)));
    commands.spawn_bundle(Camera2dBundle::default());
}
