use crate::{rtweekend::clamp, vec3::Color};
use std::io::Write;

pub fn write_color(
    out: &mut dyn Write,
    pixel_color: Color,
    samples_per_pixel: u32,
) -> std::io::Result<()> {
    let scale = 1.0 / samples_per_pixel as f64;
    let r = (256.0 * clamp((pixel_color[0] * scale).sqrt(), 0.0, 0.999)) as i32;
    let g = (256.0 * clamp((pixel_color[1] * scale).sqrt(), 0.0, 0.999)) as i32;
    let b = (256.0 * clamp((pixel_color[2] * scale).sqrt(), 0.0, 0.999)) as i32;
    writeln!(out, "{} {} {}", r, g, b)
}
