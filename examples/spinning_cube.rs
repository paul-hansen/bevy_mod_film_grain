//! A simple scene with a spinning cube with interactive sliders to control film grain settings.
use bevy::{
    color::palettes::tailwind::TEAL_900,
    feathers::{
        controls::{slider, SliderProps},
        dark_theme::create_dark_theme,
        theme::{ThemedText, UiTheme},
        FeathersPlugins,
    },
    prelude::*,
    ui_widgets::{observe, slider_self_update, SliderPrecision, SliderStep, ValueChange},
};
use bevy_mod_film_grain::{FilmGrainPlugin, FilmGrainSettings};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, FilmGrainPlugin, FeathersPlugins))
        .insert_resource(UiTheme(create_dark_theme()))
        .add_systems(Startup, (setup, setup_ui))
        .add_systems(Update, rotate)
        .run();
}

#[derive(Component)]
struct StrengthSlider;

#[derive(Component)]
struct MaxFpsSlider;

/// Set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::new(0.0, 0.0, 5.0)).looking_at(Vec3::default(), Vec3::Y),
        Camera {
            clear_color: Color::from(TEAL_900).into(),
            ..default()
        },
        // Add the setting to the camera.
        // Add this component on any cameras you want to have film grain.
        FilmGrainSettings::default(),
    ));

    // cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
        Transform::from_xyz(0.0, 0.5, 0.0),
        Rotates,
    ));
    // light
    commands.spawn(DirectionalLight {
        illuminance: 1_000.,
        ..default()
    });
}

#[derive(Component)]
struct Rotates;

/// Rotates any entity around the x and y axis
fn rotate(time: Res<Time>, mut query: Query<&mut Transform, With<Rotates>>) {
    for mut transform in &mut query {
        transform.rotate_x(0.55 * time.delta_secs());
        transform.rotate_z(0.15 * time.delta_secs());
    }
}

/// Set up the UI with sliders for changing film grain settings
fn setup_ui(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            padding: UiRect::all(Val::Px(20.0)),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(15.0),
            ..default()
        },
        children![
            (
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Start,
                    row_gap: Val::Px(10.0),
                    ..default()
                },
                children![
                    (Text::new("Strength:"), ThemedText),
                    (
                        Node {
                            width: Val::Px(200.0),
                            ..default()
                        },
                        children![
                            (
                                slider(
                                    SliderProps {
                                        min: 0.0,
                                        max: 1.0,
                                        value: 0.2,
                                    },
                                    (StrengthSlider, SliderStep(0.01), SliderPrecision(2)),
                                ),
                                observe(slider_self_update),
                                observe(
                                    |change: On<ValueChange<f32>>,
                                     mut settings: Query<&mut FilmGrainSettings>| {
                                        for mut setting in &mut settings {
                                            setting.strength = change.value;
                                        }
                                    }
                                )
                            ),
                        ]
                    ),
                ]
            ),
            (
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Start,
                    row_gap: Val::Px(10.0),
                    ..default()
                },
                children![
                    (Text::new("Max FPS:"), ThemedText),
                    (
                        Node {
                            width: Val::Px(200.0),
                            ..default()
                        },
                        children![
                            (
                                slider(
                                    SliderProps {
                                        min: 0.0,
                                        max: 120.0,
                                        value: 60.0,
                                    },
                                    (MaxFpsSlider, SliderStep(0.01), SliderPrecision(2)),
                                ),
                                observe(slider_self_update),
                                observe(
                                    |change: On<ValueChange<f32>>,
                                     mut settings: Query<&mut FilmGrainSettings>| {
                                        for mut setting in &mut settings {
                                            setting.max_fps = change.value;
                                        }
                                    }
                                )
                            ),
                        ]
                    ),
                ]
            ),
        ],
    ));
}
