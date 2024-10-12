use crate::vec3::Color;
use std::io::Write;

pub fn write_color(out: &mut dyn Write, pixel_color: Color) -> std::io::Result<()> {
    let r = (256.0 * pixel_color.x().sqrt().clamp(0.0, 0.999)) as i32;
    let g = (256.0 * pixel_color.y().sqrt().clamp(0.0, 0.999)) as i32;
    let b = (256.0 * pixel_color.z().sqrt().clamp(0.0, 0.999)) as i32;
    writeln!(out, "{} {} {}", r, g, b)
}

pub fn clamp_color(pixel_color: Color) -> String {
    let r = (256.0 * pixel_color.x().sqrt().clamp(0.0, 0.999)) as i32;
    let g = (256.0 * pixel_color.y().sqrt().clamp(0.0, 0.999)) as i32;
    let b = (256.0 * pixel_color.z().sqrt().clamp(0.0, 0.999)) as i32;
    return format!("{} {} {}", r, g, b);
}
