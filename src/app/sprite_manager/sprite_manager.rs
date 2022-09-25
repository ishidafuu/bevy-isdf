use bevy::{prelude::*, text::Text2dSize, utils::HashMap, time::Timer};
use std::time::Duration;

#[derive(Default)]
pub struct SpriteManager {
    pub sprites: HashMap<String, Handle<Image>>,

    pub unit_atlas: HashMap<String, Handle<TextureAtlas>>,
    pub unit_atlas_image: HashMap<String, Handle<Image>>,
}

// impl Default for SpriteManager {
//     // fn default() -> Self {
//     //     HitPoints { current: 1, max: 1 }
//     // }
// }


pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut sprite_manager: ResMut<SpriteManager>,
) {
    let texture_handle = asset_server.load("textures/rpg/chars/gabe/gabe-idle-run.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 7, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn_bundle(Camera2dBundle::default());
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..Default::default()
        });
}