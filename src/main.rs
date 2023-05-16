mod color;
mod vec3;

use crate::{color::write_color, vec3::Color};
fn main() {
    let image_width = 256;
    let image_height = 256;
    println!("P3\n{} {}\n255", image_width, image_height);
    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let pixel_color: Color = Color::new(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.25,
            );
            write_color(&mut std::io::stdout(), pixel_color).unwrap();
        }
    }
    eprint!("Done.");
}
