//! Shows the minimal steps required to enable this plugin.
//! Use the `spinning_cube` example if you want something pretty to look at.
use bevy::prelude::*;
use bevy_mod_film_grain::{FilmGrainPlugin, FilmGrainSettings};
fn main() {
    App::new()
        .add_plugins((DefaultPlugins, FilmGrainPlugin))
        .add_systems(Startup, setup)
        .run();
}

/// Set up a simple 3D scene
fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle::default(),
        // Add the setting to the camera.
        // This component is also used to determine on which camera to run the post processing effect.
        FilmGrainSettings::from_strength(0.05),
    ));
}
