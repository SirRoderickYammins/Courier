use bevy::prelude::*;
use bevy_fps_controller::controller::LogicalPlayer;
use bevy_rapier3d::prelude::*;

pub struct PlayerRaycast;

impl Plugin for PlayerRaycast {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, raycast);
    }
}

fn raycast(
    query: Query<(&Transform, &Collider), With<LogicalPlayer>>,
    physics_context: Res<RapierContext>,
) {
    for (transform, colliders) in query.iter() {
        // println!("Collider: {:?}", colliders);
        println!("{:?}", transform.looking_to(Vec3::Z, Vec3::Y));
    }
}
