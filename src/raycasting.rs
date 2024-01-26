use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::levels::asset_loader_plugin::Package;

pub struct PlayerRaycast;

impl Plugin for PlayerRaycast {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, raycast);
        app.add_systems(Update, display_info);
    }
}

// Lazily calling Camera transform. There's only one camera, so no need for a special marker.
// Transform has a method for getting forward vector; to be used with Rapier's raycast.

fn raycast(
    query: Query<&Transform, With<Camera>>,
    mut commands: Commands,
    _physics_context: Res<RapierContext>,
    package_query: Query<&Package>,
) {
    //Set mutable empty vec 3, will update with each loop.
    let mut ray_origin = Vec3::ZERO;

    //Get unit vector in forward direction
    let mut ray_dir = Vec3::ZERO;

    let max_toi = 4.0;

    let solid = true;

    let filter = QueryFilter::exclude_fixed();

    for camera_transform in query.iter() {
        ray_origin = camera_transform.translation;
        ray_origin.z -= 0.5;
        ray_dir = camera_transform.forward();
    }

    if let Some((entity, toi)) =
        _physics_context.cast_ray(ray_origin, ray_dir, max_toi, solid, filter)
    {
        println!("{:?}", package_query.get(entity));
        display_info(package_query.get(entity));

        // Get Entity Data here.
    }
}

fn display_info(mut commands: Commands, package: Entity) {
    commands.spawn(TextBundle::from_section(
        "Your Mother",
        TextStyle {
            font_size: 100.0,
            ..default()
        },
    ));
}
