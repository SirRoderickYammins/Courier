use std::f32::consts::{FRAC_PI_2, PI, TAU};

use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

pub struct CameraControllerPlugin;

impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_titties)
            .add_systems(Update, player_camera_transforms)
            .add_systems(Update, camera_mouse_movement);
    }
}

#[derive(Component)]
pub struct PlayerCamera {
    pub transform: Vec3,
    pub rotation: Quat,
}

#[derive(Component)]
pub struct PlayerControlInput {
    pub sprint: bool,
    pub jump: bool,
    pub crouch: bool,
    pub pitch: f32,
    pub yaw: f32,
    pub walk_speed: f32,
    pub run_speed: f32,
    pub sensitivity: f32,
}

impl Default for PlayerControlInput {
    fn default() -> Self {
        Self {
            sprint: false,
            jump: false,
            crouch: false,
            pitch: 0.0,
            yaw: 0.0,
            sensitivity: 0.01,
            walk_speed: 2.0,
            run_speed: 4.0,
        }
    }
}

impl Default for PlayerCamera {
    fn default() -> Self {
        Self {
            transform: Vec3::new(2.0, 3.0, 3.0),
            rotation: Quat::IDENTITY,
        }
    }
}

pub fn player_camera_transforms(mut cam_query: Query<(&PlayerCamera, &mut Transform)>) {
    for (camera, mut transform) in cam_query.iter_mut() {
        transform.translation = camera.transform;
        transform.rotation = camera.rotation;
    }
}

pub fn camera_mouse_movement(
    mut camera: Query<&mut PlayerControlInput>,
    mut mouse_input: EventReader<MouseMotion>,
) {
    for mut input in camera.iter_mut() {
        let mut mouse_delta = Vec2::ZERO;
        for mouse in mouse_input.read() {
            mouse_delta = mouse.delta;
        }
        mouse_delta *= input.sensitivity;
        input.yaw -= mouse_delta.x;

        input.pitch = (input.pitch - mouse_delta.y).clamp(-FRAC_PI_2, FRAC_PI_2);

        if input.yaw.abs() > PI {
            input.yaw = input.yaw.rem_euclid(TAU);
        }
    }
}

pub fn update_titties(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut cam_query: Query<(&mut PlayerCamera, &PlayerControlInput, &Transform)>,
) {
    for (mut camera, player_input, transform) in cam_query.iter_mut() {
        let forward_vector = transform.forward();
        let left_vector = transform.left();
        let right_vector = transform.right();
        let back_vector = transform.back();
        let dt = time.delta_seconds();

        let mut speed = player_input.walk_speed;

        // Walk/Run Speed
        for key in input.get_pressed() {
            match key {
                KeyCode::ShiftLeft => speed = player_input.run_speed,
                _ => (),
            }
        }

        for key in input.get_pressed() {
            match key {
                KeyCode::W => camera.transform += forward_vector * dt * speed,
                KeyCode::S => camera.transform += back_vector * dt * speed,
                KeyCode::A => camera.transform += left_vector * dt * speed,
                KeyCode::D => camera.transform += right_vector * dt * speed,
                _ => (),
            }
        }
        camera.rotation =
            Quat::from_euler(EulerRot::YXZ, player_input.yaw, player_input.pitch, 0.0);
    }
}
