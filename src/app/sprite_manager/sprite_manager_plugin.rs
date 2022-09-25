use super::sprite_manager;
use bevy::prelude::{App, Plugin};

pub struct SpriteManagerPlugin;
impl Plugin for SpriteManagerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<sprite_manager::SpriteManager>()
            .add_startup_system(sprite_manager::setup_sprite_manager)
            .add_system(sprite_manager::animate_sprite);
    }
}
