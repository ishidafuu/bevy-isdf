use super::sprite;
use bevy::prelude::{App, Plugin};

pub struct SpritePlugin;
impl Plugin for SpritePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<sprite::Sprite>()
            .add_startup_system(sprite::setup_sprite)
            .add_system(sprite::animate_sprite);
    }
}
