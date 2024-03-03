use crate::levels::asset_loader_plugin::{AssetLoaderPlugin, AssetLoaderState, MyAssetPack};
use crate::raycasting::PlayerRaycast;
use bevy::core_pipeline::bloom::BloomSettings;
use bevy::gltf::Gltf;
use bevy::pbr::DirectionalLightShadowMap;
use bevy::{prelude::*, window::CursorGrabMode};
use bevy_atmosphere::plugin::{AtmosphereCamera, AtmospherePlugin};
use bevy_fps_controller::controller::*;
use bevy_rapier3d::prelude::*;
use std::f32::consts::TAU;

use super::items::scanner::ScannerTool;

pub struct CharacterController;

impl Plugin for CharacterController {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            .add_systems(Update, (manage_cursor, respawn))
            //.add_plugins(PlayerRaycast)
            .add_plugins(AtmospherePlugin)
            .add_plugins(AssetLoaderPlugin)
            .insert_resource(DirectionalLightShadowMap { size: 4096 })
            .insert_resource(AmbientLight {
                color: Color::WHITE,
                brightness: 100.0,
            })
            .add_plugins(FpsControllerPlugin)
            .add_systems(OnEnter(AssetLoaderState::Done), setup)
            .add_plugins(ScannerTool)
            .insert_resource(ClearColor(Color::hex("D4F5F5").unwrap()));
    }
}

#[derive(Component, Clone, Copy, Debug)]
pub struct PlayerInteractionSystem {
    pub is_looking_at_item: bool,
    pub is_holding_item: bool,
    pub interactable_entity: Option<Entity>,
}

//MainChar Marker
#[derive(Component, Clone, Copy, Debug)]
pub struct MainCharacter;

const SPAWN_POINT: Vec3 = Vec3::new(0.0, 1.0, 0.0);

fn setup(
    mut commands: Commands,
    mut window: Query<&mut Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_pack: Res<MyAssetPack>,
    assets_gltf: Res<Assets<Gltf>>,
) {
    let mut window = window.single_mut();
    window.title = String::from("Courier");

    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(0., 2.5, 0.).looking_at(Vec3::new(0., 0., 0.), Vec3::Z),

        point_light: PointLight {
            intensity: 60000.0,
            color: Color::WHITE,
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });

    // Note that we have two entities for the player
    // One is a "logical" player that handles the physics computation and collision
    // The other is a "render" player that is what is displayed to the user
    // This distinction is useful for later on if you want to add multiplayer,
    // where often time these two ideas are not exactly synced up
    let logical_entity = commands
        .spawn((
            Collider::capsule(Vec3::Y * 0.5, Vec3::Y * 1.5, 0.5),
            Friction {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            Restitution {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            ActiveEvents::COLLISION_EVENTS,
            Velocity::zero(),
            RigidBody::Dynamic,
            Sleeping::disabled(),
            LockedAxes::ROTATION_LOCKED,
            AdditionalMassProperties::Mass(1.0),
            GravityScale(0.0),
            Ccd { enabled: true }, // Prevent clipping when going fast
            TransformBundle::from_transform(Transform::from_translation(SPAWN_POINT)),
            LogicalPlayer,
            FpsControllerInput {
                pitch: -TAU / 12.0,
                yaw: TAU * 5.0 / 8.0,
                ..default()
            },
            FpsController {
                air_acceleration: 80.0,
                ..default()
            },
        ))
        .insert(CameraConfig {
            height_offset: 0.0,
            radius_scale: 0.75,
        })
        .id();

    if let Some(scanner) = assets_gltf.get(&asset_pack.scanner) {
        if let Some(player_hand) = assets_gltf.get(&asset_pack.player_hand) {
            commands
                .spawn((
                    Camera3dBundle {
                        camera: Camera {
                            hdr: true,
                            ..default()
                        },
                        projection: Projection::Perspective(PerspectiveProjection {
                            fov: TAU / 5.0,
                            ..default()
                        }),
                        ..default()
                    },
                    PlayerInteractionSystem {
                        is_holding_item: false,
                        is_looking_at_item: false,
                        interactable_entity: None,
                    },
                    BloomSettings::OLD_SCHOOL,
                    RenderPlayer { logical_entity },
                    AtmosphereCamera::default(),
                ))
                .with_children(|cam| {
                    cam.spawn((
                        SceneBundle {
                            scene: scanner.named_scenes["Scene"].clone(),
                            transform: Transform::from_xyz(0.3, -0.2, -0.5),

                            ..default()
                        },
                        ScannerTool,
                    ));
                })
                .with_children(|cam| {
                    cam.spawn(SceneBundle {
                        scene: player_hand.named_scenes["Scene"].clone(),
                        transform: Transform::from_xyz(0.4, -0.7, -0.4),
                        ..default()
                    });
                });
        }
    }
}

fn respawn(mut query: Query<(&mut Transform, &mut Velocity)>) {
    for (mut transform, mut velocity) in &mut query {
        if transform.translation.y > -50.0 {
            continue;
        }

        velocity.linvel = Vec3::ZERO;
        transform.translation = SPAWN_POINT;
    }
}

fn manage_cursor(
    btn: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
    mut window_query: Query<(Entity, &mut Window)>,
    mut controller_query: Query<&mut FpsController>,
) {
    let mut window = window_query.single_mut();
    let center = Some(Vec2::new(window.1.width() / 2.0, window.1.height() / 2.0));
    if window.1.cursor.grab_mode == CursorGrabMode::Locked {
        window.1.set_cursor_position(center);
    }
    if btn.just_pressed(MouseButton::Left) {
        window.1.cursor.grab_mode = CursorGrabMode::Locked;
        window.1.cursor.visible = false;
        for mut controller in &mut controller_query {
            controller.enable_input = true;
        }
    }
    if key.just_pressed(KeyCode::Escape) {
        window.1.cursor.grab_mode = CursorGrabMode::None;
        window.1.cursor.visible = true;
        for mut controller in &mut controller_query {
            controller.enable_input = false;
        }
    }
}
