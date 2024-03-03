use bevy::{prelude::*, ui::SetUiViewBindGroup};
use bevy_framepace::Limiter;
use bevy_rapier3d::prelude::RapierDebugRenderPlugin;
use courier::player::controller::CharacterController;
use std::env;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(CharacterController)
        .add_plugins(bevy_framepace::FramepacePlugin)
        .add_systems(Update, frame_pace)
        .run();

    env::set_var("RUST_BACKTRACE", "1");
}

fn frame_pace(mut settings: ResMut<bevy_framepace::FramepaceSettings>) {
    settings.limiter = Limiter::Auto;
}
