/// Linear interpolation between two floats
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t.clamp(0.0, 1.0)
}

/// Linear interpolation between two [f32; 2] vectors
pub fn lerp_vec2(a: [f32; 2], b: [f32; 2], t: f32) -> [f32; 2] {
    [
        lerp(a[0], b[0], t),
        lerp(a[1], b[1], t),
    ]
}

/// Maps a value from one range to another
pub fn map_range(val: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
    let normalized = (val - in_min) / (in_max - in_min);
    out_min + normalized * (out_max - out_min)
}
