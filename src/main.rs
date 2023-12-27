use bevy::{prelude::*, window::CursorGrabMode};
use bevy_atmosphere::prelude::*;
use bevy_rapier3d::{control::KinematicCharacterController, dynamics::RigidBody, prelude::*};
use courier::controller::*;
use std::f32::consts::PI;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, AtmospherePlugin))
        .add_systems(Startup, setup)
        .insert_resource(RapierConfiguration::default())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_systems(Startup, spawn_gltf)
        .add_systems(Update, grab_mouse)
        .add_plugins(CameraControllerPlugin)
        .add_systems(Update, player_pos)
        .run();
}

fn spawn_gltf(mut commands: Commands, ass: Res<AssetServer>) {
    let my_gltf = ass.load("../assets/Apartment 2.glb#Scene0");

    commands
        .spawn((
            RigidBody::Fixed,
            SceneBundle {
                scene: my_gltf,
                transform: Transform::from_xyz(0.0, -3.0, 0.0),
                ..default()
            },
        ))
        .insert(Collider::cuboid(5.0, 5.0, 5.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -3.0, 0.0)));
}

fn setup(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 8.0, 0.0),
        ..default()
    });

    commands
        .spawn((
            Collider::ball(3.5),
            RigidBody::Dynamic,
            Ccd { enabled: true },
            Camera3dBundle {
                projection: Projection::Perspective(PerspectiveProjection {
                    fov: PI / 2.5,
                    ..default()
                }),
                ..default()
            },
            PlayerCamera { ..default() },
            PlayerControlInput { ..default() },
            AtmosphereCamera::default(),
        ))
        .insert(KinematicCharacterController::default());

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

fn player_pos(
    player_queer: Query<&KinematicCharacterController>,
    mut text_query: Query<&mut Text>,
) {
    for player in player_queer.iter() {
        println!("Niggers: {:?}", player.translation);
    }
}
