use bevy::{prelude::*, window::CursorGrabMode};
use bevy_atmosphere::prelude::*;
use courier::controller::*;
use std::f32::consts::PI;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, AtmospherePlugin))
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_gltf)
        .add_systems(Update, grab_mouse)
        .add_plugins(CameraControllerPlugin)
        .run();
}

fn spawn_gltf(mut commands: Commands, ass: Res<AssetServer>) {
    let my_gltf = ass.load("../assets/Apartment 2.glb#Scene0");

    commands.spawn(SceneBundle {
        scene: my_gltf,
        transform: Transform::from_xyz(0.0, 1.0, -4.0),
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

    commands.spawn((
        Camera3dBundle {
            projection: Projection::Perspective(PerspectiveProjection {
                fov: PI / 3.0,
                ..default()
            }),
            ..default()
        },
        PlayerCamera { ..default() },
        PlayerControlInput { ..default() },
        AtmosphereCamera::default(),
    ));

    commands.spawn(
        TextBundle::from_section(
            "Poo Poo Pee Pee",
            TextStyle {
                font_size: 24.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        }),
    );
}

fn grab_mouse(
    mut windows: Query<&mut Window>,
    mouse: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
) {
    let mut window = windows.single_mut();

    if mouse.just_pressed(MouseButton::Left) {
        window.cursor.visible = false;
        window.cursor.grab_mode = CursorGrabMode::Locked;
    }

    if key.just_pressed(KeyCode::Escape) {
        window.cursor.visible = true;
        window.cursor.grab_mode = CursorGrabMode::None;
    }
}
