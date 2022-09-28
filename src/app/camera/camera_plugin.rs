use bevy::{prelude::*};

use super::camera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(camera::setup_camera);
    }
}
