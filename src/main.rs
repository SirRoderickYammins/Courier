use bevy::prelude::*;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_gltf)
        .run();
}

fn spawn_gltf(mut commands: Commands, ass: Res<AssetServer>) {
    let my_gltf = ass.load("../assets/Apartment 2.glb#Scene0");
    let cute_dino = ass.load("../assets/dino.glb#Scene1");

    commands.spawn(SceneBundle {
        scene: cute_dino,
        transform: Transform::from_xyz(2.0, 2.4, 4.0),
        ..default()
    });

    commands.spawn(SceneBundle {
        scene: my_gltf,
        transform: Transform::from_xyz(2.0, 1.0, 4.0),
        ..default()
    });
}

fn setup(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    commands.spawn(Camera3dBundle::default());
}
