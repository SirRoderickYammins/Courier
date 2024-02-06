// This file will load the beginning warehouse. Functions to load .glb master
// assets will be created in a general format to be applied in any manner.

use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::levels::package_data::Package;
use crate::tools::gltf::GltfToolsPlugin;

pub struct AssetLoaderPlugin;
impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AssetLoaderState>()
            .add_loading_state(
                LoadingState::new(AssetLoaderState::Loading)
                    .continue_to_state(AssetLoaderState::Done)
                    .load_collection::<MyAssetPack>(),
            )
            .add_systems(OnEnter(AssetLoaderState::Done), load_scene)
            .add_systems(Update, spawn_box.run_if(in_state(AssetLoaderState::Done)))
            .add_plugins(GltfToolsPlugin);
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

#[derive(AssetCollection, Resource, Debug)]
pub struct MyAssetPack {
    #[asset(path = "starting_warehouse.glb")]
    pub main_scene: Handle<Gltf>,
    #[asset(path = "box.glb")]
    pub package: Handle<Gltf>,
}

// Extract mesh from GLTF in order to have Rapier compute a collider shape. General function
// structure allows for multiple models and levels to bew made with ease.

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
                On::<Pointer<Click>>::run(|event: Listener<Pointer<Click>>| {
                    info!("Clicked on box {:?}", event.target);
                }),
            ));
        }
    }
}
