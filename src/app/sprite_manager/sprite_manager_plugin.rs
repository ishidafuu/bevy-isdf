use bevy::prelude::{App, Plugin};
use crate::app::sprite_manager::sprite_manager;

use super::sprite_manager::*;

pub struct SpriteManagerPlugin;
impl Plugin for SpriteManagerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpriteManager>();
    }
}
