use bevy::prelude::*;

mod camera;
mod layout;
mod sprite_manager;
mod state;

pub fn run() {
    let mut app = App::new();

    app.insert_resource(setup_window())
        .add_plugins(DefaultPlugins)
        .add_plugin(camera::camera_plugin::SetupCameraPlugin)
        // .add_plugin(state::state_plugin::SwitchStatePlugin)
        .add_plugin(layout::layout_plugin::AppLayoutPlugin)
        .add_plugin(sprite_manager::sprite_manager_plugin::SpriteManagerPlugin);

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
