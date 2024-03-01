use crate::levels::package_data::Package;
use crate::player::controller::PlayerInteractionSystem;
use bevy::math::Vec2;
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
//

fn get_viewport_center(window: &Window) -> Vec2 {
    let x = window.resolution.width() / 2.0;
    let y = window.resolution.height() / 2.0;

    let center_point = Vec2::new(x, y);

    center_point
}

//Raycast constantly updates, modiftying the player's interaction system resource (defined in
//controller.rs).
fn raycast(
    window_query: Query<&Window>,
    mut camera_query: Query<(&Camera, &GlobalTransform, &mut PlayerInteractionSystem)>,
    _physics_context: Res<RapierContext>,
) {
    let qf = QueryFilter::new();
    let window = window_query.single();
    //Set mutable empty vec 3, will update with each loop.
    let max_toi = 5.0;
    let solid = false;
    let filter = qf.groups(CollisionGroups {
        memberships: Group::GROUP_2,
        filters: Group::GROUP_2,
    });
    for (camera, camera_transform, mut player_interaction_sys) in camera_query.iter_mut() {
        let Some(mut ray) = camera.viewport_to_world(camera_transform, get_viewport_center(window))
        else {
            return;
        };

        ray.origin.z -= 1.0;

        if let Some((entity, _toi)) =
            _physics_context.cast_ray(ray.origin, ray.direction, max_toi, solid, filter)
        {
            player_interaction_sys.is_looking_at_item = true;
            player_interaction_sys.interactable_entity = Some(entity);
        } else {
            player_interaction_sys.is_looking_at_item = false;
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
