use bevy::prelude::*;

mod camera;
mod input;
mod layout;
mod sprite;
mod state;

pub fn run() {
    let mut app = App::new();

    app.insert_resource(setup_window())
        .add_plugins(DefaultPlugins)
        .add_plugin(camera::camera::CameraPlugin)
        // .add_plugin(state::state_plugin::SwitchStatePlugin)
        .add_plugin(layout::layout::AppLayoutPlugin)
        .add_plugin(sprite::sprite::SpritePlugin)
        .add_plugin(input::input::InputPlugin);

    // // when building for Web, use WebGL2 rendering
    // #[cfg(target_arch = "wasm32")]
    //     app.add_plugin(bevy_webgl2::WebGL2Plugin);
    app.run();
}

// ウインドウのセットアップ
fn setup_window() -> WindowDescriptor {
    WindowDescriptor {
        title: "My Bevy App".to_string(),
        width: 640.,
        height: 480.,
        #[cfg(target_arch = "wasm32")]
        canvas: Some("canvas".to_string()),
        ..Default::default()
    }
}
