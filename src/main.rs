use bevy::{prelude::*, window::CursorGrabMode};
use bevy_atmosphere::prelude::*;
use bevy_rapier3d::{control::KinematicCharacterController, prelude::*};
use courier::controller::*;
use courier::get_scene_colliders;
use std::f32::consts::PI;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, AtmospherePlugin))
        .add_state::<GameState>()
        .insert_resource(RapierConfiguration::default())
        .add_plugins((
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
        ))
        .add_systems(OnEnter(GameState::Loading), (spawn_gltf,))
        .add_systems(Update, grab_mouse)
        .add_plugins(CameraControllerPlugin)
        .add_systems(
            Update,
            (check_if_loaded,).run_if(in_state(GameState::Loading)),
        )
        .add_systems(OnEnter(GameState::Loaded), setup)
        .run();
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, States, Default)]

enum GameState {
    #[default]
    Loading,
    Loaded,
}

#[derive(Default, Resource)]
struct GameAssets {
    apartment_scene: Handle<Scene>,
    apartment_collider: Vec<(Collider, Transform)>,
}

// Loading apartment scene from gltf
fn spawn_gltf(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        apartment_scene: asset_server.load("../assets/untitled.glb#Scene0"),
        ..default()
    });
}
//Check if scene is loaded, then grab colliders
fn check_if_loaded(
    mut scenes: ResMut<Assets<Scene>>,
    mut game_assets: ResMut<GameAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    let scene = if let Some(scene) = scenes.get_mut(&game_assets.apartment_scene) {
        scene
    } else {
        return;
    };

    // Call get scene colliders. Should only be called once otherwise it will remove collider
    // meshes
    game_assets.apartment_collider = get_scene_colliders(&mut meshes, &mut scene.world)
        .expect("Failed to create apartment colliders");

    game_state.set(GameState::Loaded);
}

fn setup(mut commands: Commands, game_assets: Res<GameAssets>) {
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
            RigidBody::Fixed,
            Sleeping::disabled(),
            SceneBundle {
                scene: game_assets.apartment_scene.clone(),
                transform: Transform::from_translation(Vec3::new(0.0, -5.0, 0.0)),
                ..default()
            },
        ))
        .with_children(|parent| {
            for (collider, transform) in game_assets.apartment_collider.iter() {
                parent.spawn((
                    collider.clone(),
                    TransformBundle::from_transform(*transform),
                ));
            }
        });

    commands
        .spawn((
            Collider::ball(0.25),
            RigidBody::Dynamic,
            Ccd { enabled: true },
            GravityScale(1.0),
            Sleeping::disabled(),
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
