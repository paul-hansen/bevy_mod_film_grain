# Bevy Film Grain Shader

A film grain post process shader implementation for the [Bevy game engine](https://bevyengine.org).
It can be used with a single camera or multiple.
It's tested to work on desktop and the web with WebGL 2.
WebGPU on the web should work but I haven't tested it yet.

![image](https://github.com/paul-hansen/bevy_mod_film_grain/assets/7019130/9c232a48-e914-43f8-bb2f-f8da8bbd6025)


## Minimal Example

```rust
use bevy::prelude::*;
use bevy_mod_film_grain::{FilmGrainPlugin, FilmGrainSettings};
fn main() {
    App::new()
        .add_plugins((DefaultPlugins, FilmGrainPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle::default(),
        // Adding this component enables the effect for this camera only.
        FilmGrainSettings::from_strength(0.2),
    ));
}
```

## License

Licensed under either of

 * [Apache License, Version 2.0](<http://www.apache.org/licenses/LICENSE-2.0>)
 * [MIT license](<http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
