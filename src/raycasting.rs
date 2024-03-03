use bevy::math::Vec2;
use bevy::prelude::*;
pub struct PlayerRaycast;

impl Plugin for PlayerRaycast {
    fn build(&self, app: &mut App) {}
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
