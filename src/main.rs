use bevy::{gltf::Gltf, gltf::GltfMesh, gltf::GltfNode, prelude::*, window::CursorGrabMode};
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
        .add_systems(Update, setup_collision_for_imported_meshes)
        .run();
}

#[derive(Bundle)]
struct MainMeshBundle {
    collider: Collider,
    rigid_body: RigidBody,
    transform: TransformBundle,
}

#[derive(Component)]
struct ImportedGLTF;

fn spawn_gltf(mut commands: Commands, asset_server: Res<AssetServer>) {
    let gltf_scene: Handle<Scene> = asset_server.load("../assets/Apartment 2.glb#Scene0");

    commands
        .spawn(SceneBundle {
            scene: gltf_scene,
            transform: Transform::from_xyz(0.0, -5.0, 0.0),
            ..default()
        })
        .insert(ImportedGLTF);
}

fn setup_collision_for_imported_meshes(
    mut commands: Commands,
    query: Query<&Handle<Mesh>, With<ImportedGLTF>>,
    meshes: ResMut<Assets<Mesh>>,
) {
    for mesh_handle in query.iter() {
        if let Some(mesh) = meshes.get(&*mesh_handle) {
            let collider = Collider::from_bevy_mesh(mesh, &ComputedColliderShape::TriMesh)
                .expect("Couldn't load mesh");
            commands
                .spawn(MainMeshBundle {
                    collider,
                    rigid_body: RigidBody::Fixed,
                    transform: TransformBundle::from_transform(Transform::from_xyz(0.0, -5.0, 0.0)),
                })
                .insert(ActiveCollisionTypes::KINEMATIC_STATIC);
        }
    }
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
            Collider::ball(0.5),
            RigidBody::Dynamic,
            ActiveCollisionTypes::DYNAMIC_KINEMATIC,
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
