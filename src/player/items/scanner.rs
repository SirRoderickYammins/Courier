use bevy::prelude::*;
use bevy_fps_controller::controller::LogicalPlayer;
use bevy_rapier3d::prelude::Velocity;

#[derive(Component, Debug)]
pub struct ScannerTool;

impl Plugin for ScannerTool {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, scanner_sway);
    }
}

fn scanner_sway(
    mut scanner_query: Query<&mut Transform, With<ScannerTool>>,
    time: Res<Time>,
    player_query: Query<&Velocity, With<LogicalPlayer>>,
) {
    let step = 6.0;

    for player_velocity in player_query.iter() {
        if player_velocity.linvel != Vec3::ZERO {
            for mut scanner_transform in scanner_query.iter_mut() {
                let time_since_start = time.elapsed_seconds();
                let delta: f32 = (time_since_start * step).sin();
                scanner_transform.translation.y += delta / 1500.0;
            }
        }
    }
}
