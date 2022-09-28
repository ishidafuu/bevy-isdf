use bevy::{prelude::*};

use super::input;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(input::setup_input);
    }
}
