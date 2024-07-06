#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
struct FilmGrainSettings {
    strength: f32,
#ifdef SIXTEEN_BYTE_ALIGNMENT
    _webgl2_padding: vec3<f32>,
#endif
}
@group(0) @binding(2) var<uniform> settings: FilmGrainSettings;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    // Chromatic aberration strength
    let noise = (fract(sin(dot(in.uv, vec2(120.9898, 78.233))) * 43758.5453) - 0.5) * 2.0;
    let color = textureSample(screen_texture, texture_sampler, in.uv);
    let color_grainy = color + (noise * settings.strength);
    // Sample each color channel with an arbitrary shift
    return color_grainy;
}

