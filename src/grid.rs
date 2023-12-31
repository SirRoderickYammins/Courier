use bevy::prelude::*;

pub struct GridLines;

impl Plugin for GridLines {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_grid);
    }
}

pub fn create_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Mesh::from(shape::Plane {
        size: 2.,
        ..default()
    }));
    let white_material = materials.add(Color::rgb(1., 0.9, 0.9).into());
    let black_material = materials.add(Color::rgb(0., 0.1, 0.1).into());

    for i in 0..32 {
        for j in 0..32 {
            commands.spawn(PbrBundle {
                mesh: mesh.clone(),
                material: if (i + j + 1) % 2 == 0 {
                    white_material.clone()
                } else {
                    black_material.clone()
                },

                transform: Transform::from_translation(Vec3::new(
                    2.0 * i as f32 - 32.0,
                    0.01,
                    2.0 * j as f32 - 32.0,
                )),
                ..default()
            });
        }
    }
}
