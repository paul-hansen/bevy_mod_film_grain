# Bevy Film Grain Shader

This is a film grain post process shader implementation for the [Bevy game engine](https://bevyengine.org).
It can be used with a single camera or multiple.
It's tested to work on desktop, wasm with webgl2. WebGpu should work but I haven't tested it yet.


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
        FilmGrainSettings::new(0.05, 1.0),
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
