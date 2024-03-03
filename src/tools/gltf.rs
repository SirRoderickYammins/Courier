use bevy::{
    gltf::{Gltf, GltfMesh, GltfNode},
    prelude::*,
};
use bevy_rapier3d::prelude::*;

use crate::levels::asset_loader_plugin::{AssetLoaderState, MyAssetPack};

pub struct GltfToolsPlugin;

impl Plugin for GltfToolsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AssetLoaderState::Done), generate_gltf_colliders);
    }
}

pub fn generate_gltf_colliders(
    mut commands: Commands,
    asset_pack: ResMut<MyAssetPack>,
    gltf_assets: Res<Assets<Gltf>>,
    gltf_mesh_assets: Res<Assets<GltfMesh>>,
    gltf_node_assets: Res<Assets<GltfNode>>,
    mesh_assets: Res<Assets<Mesh>>,
) {
    let gltf = gltf_assets.get(&asset_pack.main_scene);

    for node in &gltf.unwrap().nodes {
        let node = gltf_node_assets.get(node).unwrap();
        if let Some(gltf_mesh) = node.mesh.clone() {
            let gltf_mesh = gltf_mesh_assets.get(&gltf_mesh).unwrap();
            for mesh_primitive in &gltf_mesh.primitives {
                let mesh = mesh_assets.get(&mesh_primitive.mesh).unwrap();
                commands.spawn((
                    Collider::from_bevy_mesh(mesh, &ComputedColliderShape::TriMesh).unwrap(),
                    RigidBody::Fixed,
                    TransformBundle::from_transform(node.transform),
                ));
            }
        }
    }
}
