//! A simple scene with a spinning cube. The film grain amount fades in and out over time.
use bevy::{color::palettes::tailwind::TEAL_900, prelude::*};
use bevy_mod_film_grain::{FilmGrainPlugin, FilmGrainSettings};
fn main() {
    App::new()
        .add_plugins((DefaultPlugins, FilmGrainPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, (rotate))
        .run();
}

/// Set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 5.0))
                .looking_at(Vec3::default(), Vec3::Y),
            camera: Camera {
                clear_color: Color::from(TEAL_900).into(),
                ..default()
            },
            ..default()
        },
        // Add the setting to the camera.
        // This component is also used to determine on which camera to run the post processing effect.
        FilmGrainSettings::default(),
    ));

    // cube
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::default()),
            material: materials.add(Color::srgb(0.8, 0.7, 0.6)),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Rotates,
    ));
    // light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 1_000.,
            ..default()
        },
        ..default()
    });
}

#[derive(Component)]
struct Rotates;

/// Rotates any entity around the x and y axis
fn rotate(time: Res<Time>, mut query: Query<&mut Transform, With<Rotates>>) {
    for mut transform in &mut query {
        transform.rotate_x(0.55 * time.delta_seconds());
        transform.rotate_z(0.15 * time.delta_seconds());
    }
}

// Change the intensity over time to show that the effect is controlled from the main world
fn update_settings(mut settings: Query<&mut FilmGrainSettings>, time: Res<Time>) {
    for mut setting in &mut settings {
        let mut strength = time.elapsed_seconds().sin();
        // Make it loop periodically
        strength = strength.sin();
        // Remap it to 0..1 because the intensity can't be negative
        strength = strength * 0.5 + 0.5;
        // Scale it to a more reasonable level
        strength *= 0.018;

        // Set the intensity.
        // This will then be extracted to the render world and uploaded to the gpu automatically by the [`UniformComponentPlugin`]
        setting.strength = strength;
    }
}
