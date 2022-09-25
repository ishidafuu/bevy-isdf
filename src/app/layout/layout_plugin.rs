use bevy::{prelude::*};

use super::score_board;

pub struct AppLayoutPlugin;

impl Plugin for AppLayoutPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(score_board::setup);
    }
}
