// This file will load the beginning warehouse. Functions to load .glb master
// assets will be created in a general format to be applied in any manner.

use bevy::gltf::Gltf;
use bevy::prelude::*;

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
            .add_systems(Update, spawn_box.run_if(in_state(AssetLoaderState::Done)));
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

#[derive(Resource, Debug)]
pub struct MyAssetPack(pub Handle<Gltf>);

const ASSET_PATH: &str = "starting_warehouse.glb";

fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    //Load the asset, store the handle in the MyAssetPack struct.

    let gltf: Handle<Gltf> = asset_server.load(ASSET_PATH);

    commands.insert_resource(MyAssetPack(gltf));
}

fn check_load_complete(
    asset_pack: Res<MyAssetPack>,
    mut next_state: ResMut<NextState<AssetLoaderState>>,
    mut asset_events: EventReader<AssetEvent<Gltf>>,
) {
    for event in asset_events.read() {
        if event.is_loaded_with_dependencies(asset_pack.0.clone()) {
            next_state.set(AssetLoaderState::Done);
            println!("Asset Loaded");
        }
    }
}

#[derive(Debug, Resource)]
pub struct BoxComponent(pub Handle<Scene>);

fn load_scene(
    mut commands: Commands,
    asset_pack: Res<MyAssetPack>,
    assets_gltf: Res<Assets<Gltf>>,
) {
    if let Some(gltf) = assets_gltf.get(&asset_pack.0) {
        println!("Named scenes: {:?}", gltf.named_scenes);
        commands.spawn(SceneBundle {
            scene: gltf.named_scenes["Scene"].clone(),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        });
        commands.insert_resource(BoxComponent(gltf.named_scenes["Scene.001"].clone()));
    }
}

fn spawn_box(
    mut commands: Commands,
    my_box_component: Res<BoxComponent>,
    input: Res<Input<KeyCode>>,
) {
    if input.pressed(KeyCode::G) {
        commands.spawn(SceneBundle {
            scene: my_box_component.0.clone(),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        });
    }
}
