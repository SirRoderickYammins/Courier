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
    mut package_query: Query<(&Package, &mut Transform), (With<Package>, Without<Camera>)>,
    mut text_query: Query<&mut Text, With<PackageInfoText>>,
    input: Res<Input<KeyCode>>,
) {
    //Set mutable empty vec 3, will update with each loop.
    let mut ray_origin = Vec3::ZERO;
    //Get unit vector in forward direction
    let mut ray_dir = Vec3::ZERO;
    let max_toi = 4.0;
    let solid = true;
    let filter = QueryFilter::exclude_fixed();

    let mut playerholding = false;

    for camera_transform in query.iter() {
        ray_origin = camera_transform.translation;
        ray_origin.z -= 0.5;
        ray_dir = camera_transform.forward();
    }

    if let Some((entity, _toi)) =
        _physics_context.cast_ray(ray_origin, ray_dir, max_toi, solid, filter)
    {
        if let Ok((package_item, mut transform)) = package_query.get_mut(entity) {
            for mut text in &mut text_query {
                text.sections[0].value = format!(
                    "Name: {}\nAddr:{}\nCountry:{}\nZIP:{}\nWeight:{:.2}",
                    package_item.recipient_name,
                    package_item.street_address,
                    package_item.country,
                    package_item.zip_code,
                    package_item.weight,
                );
            }

            if input.pressed(KeyCode::E) {
                playerholding = true;
            }

            if playerholding {
                ray_origin.z -= 2.0;
                transform.translation = ray_origin;
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
