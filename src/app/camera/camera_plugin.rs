use bevy::{prelude::*};

use super::camera;

pub struct SetupCameraPlugin;

impl Plugin for SetupCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(camera::setup_camera);
    }
}
