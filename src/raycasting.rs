use crate::levels::package_data::Package;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct PlayerRaycast;

impl Plugin for PlayerRaycast {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, raycast);
        app.add_systems(Startup, display_info);
    }
}

// Lazily calling Camera transform. There's only one camera, so no need for a special marker.
// Transform has a method for getting forward vector; to be used with Rapier's raycast.

fn raycast(
    query: Query<&Transform, With<Camera>>,
    _physics_context: Res<RapierContext>,
    package_query: Query<&Package>,
    mut text_query: Query<&mut Text, With<PackageInfoText>>,
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

    if let Some((entity, _toi)) =
        _physics_context.cast_ray(ray_origin, ray_dir, max_toi, solid, filter)
    {
        if let Ok(package_ent) = package_query.get(entity) {
            for mut text in &mut text_query {
                text.sections[0].value = format!(
                    "Name: {}\nAddr:{}\nCountry:{}\nZIP:{}\nWeight:{:.2}",
                    package_ent.recipient_name,
                    package_ent.street_address,
                    package_ent.country,
                    package_ent.zip_code,
                    package_ent.weight
                );
            }
        }
    }
}

#[derive(Component)]
pub struct PackageInfoText;

fn display_info(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
            "",
            TextStyle {
                font_size: 40.0,
                ..default()
            },
        ),
        PackageInfoText,
    ));
}
