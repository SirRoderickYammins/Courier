// This file will load the beginning warehouse. Functions to load .glb master
// assets will be created in a general format to be applied in any manner.

use bevy::gltf::Gltf;
use bevy::math::vec4;
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::levels::package_data::Package;

pub struct AssetLoaderPlugin;
impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AssetLoaderState>()
            .add_systems(OnEnter(AssetLoaderState::Loading), load_assets)
            .add_systems(
                Update,
                check_load_complete.run_if(in_state(AssetLoaderState::Loading)),
            )
            .add_systems(OnEnter(AssetLoaderState::Done), load_scene)
            .add_systems(Update, spawn_box.run_if(in_state(AssetLoaderState::Done)))
            .add_systems(OnEnter(AssetLoaderState::Done), generate_colliders)
            .add_plugins(DefaultPickingPlugins);
    }
}

// Deriving an enum that will track whether the GLTF is loaded.
// The file must be loaded before we can continue.

#[derive(Default, Clone, Eq, PartialEq, Hash, States, Debug)]
pub enum AssetLoaderState {
    #[default]
    Loading,
    Done,
}

// I found it was best to have one central resource to contain all GLTF files to be accessed.
// It is also easier to have separate GLTF files for each entity that you wish to spawn.

#[derive(Resource, Debug)]
pub struct MyAssetPack {
    main_scene: Handle<Gltf>,
    package: Handle<Gltf>,
}

const ASSET_PATH: &str = "starting_warehouse.glb";

fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    //Load the asset, store the handle in the MyAssetPack struct.

    let gltf: Handle<Gltf> = asset_server.load(ASSET_PATH);
    let package: Handle<Gltf> = asset_server.load("box.glb");

    commands.insert_resource(MyAssetPack {
        main_scene: gltf,
        package,
    });
}

// Extract mesh from GLTF in order to have Rapier compute a collider shape. General function
// structure allows for multiple models and levels to bew made with ease.

fn check_load_complete(
    asset_pack: Res<MyAssetPack>,
    mut next_state: ResMut<NextState<AssetLoaderState>>,
    mut asset_events: EventReader<AssetEvent<Gltf>>,
) {
    for event in asset_events.read() {
        if event.is_loaded_with_dependencies(asset_pack.main_scene.clone()) {
            next_state.set(AssetLoaderState::Done);
            println!("Asset Loaded");
        }
    }
}

fn load_scene(
    mut commands: Commands,
    asset_pack: Res<MyAssetPack>,
    assets_gltf: Res<Assets<Gltf>>,
) {
    if let Some(gltf) = assets_gltf.get(&asset_pack.main_scene) {
        commands.spawn(SceneBundle {
            scene: gltf.named_scenes["Scene"].clone(),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        });
    }
}

fn spawn_box(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    asset_pack: Res<MyAssetPack>,
    assets_gltf: Res<Assets<Gltf>>,
) {
    if let Some(gltf) = assets_gltf.get(&asset_pack.package) {
        if input.pressed(KeyCode::G) {
            commands.spawn((
                SceneBundle {
                    scene: gltf.named_scenes["Scene"].clone(),
                    transform: Transform::from_xyz(0., 2.5, 0.),
                    ..default()
                },
                Collider::cuboid(0.5, 0.5, 0.5),
                Friction::coefficient(1.2),
                RigidBody::Dynamic,
                Dominance::group(0),
                Package::new(),
                PickableBundle::default(),
                HIGHLIGHT_TINT,
                On::<Pointer<Click>>::run(|event: Listener<Pointer<Click>>| {
                    info!("Clicked on box {:?}", event.target);
                }),
            ));
        }
    }
}

const HIGHLIGHT_TINT: Highlight<StandardMaterial> = Highlight {
    hovered: Some(HighlightKind::new_dynamic(|matl| StandardMaterial {
        base_color: matl.base_color + vec4(-0.5, -0.3, 0.9, 0.8),
        ..matl.to_owned()
    })),
    pressed: Some(HighlightKind::new_dynamic(|matl| StandardMaterial {
        base_color: matl.base_color + vec4(-0.4, -0.4, 0.8, 0.8),
        ..matl.to_owned()
    })),
    selected: Some(HighlightKind::new_dynamic(|matl| StandardMaterial {
        base_color: matl.base_color + vec4(-0.4, 0.8, -0.4, 0.0),
        ..matl.to_owned()
    })),
};

#[derive(Bundle, Debug)]
struct ColliderBundle {
    collider_shape: Collider,
    rigid_body_type: RigidBody,
    transform: TransformBundle,
}

#[derive(Component, Debug)]
pub struct MainSceneCollider;

//TODO: Create either an Enum/Struct in a separate file (for cleanliness) to contain
//the transforms and sizes of the colliders for the walls and other map surfaces.

fn generate_colliders(mut commands: Commands) {
    commands.spawn((
        MainSceneCollider,
        ColliderBundle {
            collider_shape: Collider::cuboid(8.0, 0.1, 8.0),
            rigid_body_type: RigidBody::Fixed,
            transform: TransformBundle::from(Transform::from_xyz(0., -0.1, 0.)),
        },
    ));
}
