use bevy::prelude::{App, Plugin};

use super::sprite_manager::*;

pub struct SpriteManagerPlugin;
impl Plugin for SpriteManagerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpriteManager>();
    }
}
