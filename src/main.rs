use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use courier::player::controller::CharacterController;
use std::env;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        //.add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(CharacterController)
        .run();

    env::set_var("RUST_BACKTRACE", "1");
}
