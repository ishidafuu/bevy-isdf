use bevy::prelude::{App, IntoSystem, Plugin, State, SystemSet};

use super::sprite_manager::*;

pub struct SpriteManagerPlugin;
impl Plugin for SpriteManagerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpriteManager>();
    }
}
