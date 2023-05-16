use crate::vec3::Color;
use std::io::Write;

pub fn write_color(out: &mut dyn Write, pixel_color: Color) -> std::io::Result<()> {
    writeln!(
        out,
        "{} {} {}",
        (255.999 * pixel_color.x()) as u8,
        (255.999 * pixel_color.y()) as u8,
        (255.999 * pixel_color.z()) as u8
    )
}
