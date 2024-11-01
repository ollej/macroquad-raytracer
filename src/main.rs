use macroquad_raytracer::prelude::*;

use clap::Parser;
use std::f32::consts::PI;

fn generate_clock() -> Canvas {
    let half_width = 50.0;
    let mut canvas = canvas((half_width * 2.0) as usize, (half_width * 2.0) as usize);
    let origin = point(0.0, 0.0, 0.0);
    let twelve = point(0.0, 0.0, 1.0);
    let red = color(1.0, 0.0, 0.0);
    let radius = half_width * 2.0 * (3.0 / 8.0);

    for h in 0..12 {
        let r = rotation_y(h as f32 * (PI / 6.0));
        let hour = r * twelve;
        let x = ((hour.x + origin.x) * radius + half_width).round() as usize;
        let y = ((hour.z + origin.z) * radius + half_width).round() as usize;

        canvas.write_pixel(x, y, &red);
    }

    canvas
}

#[macroquad::main(window_conf())]
async fn main() {
    let options = AppOptions::parse();

    let c = generate_clock();
    let image = c.as_image();

    match options.image {
        Some(ImageType::PNG) => save_png(&image, &options.image_path().unwrap()),
        Some(ImageType::PPM) => c.save_ppm(&options.image_path().unwrap()),
        None => (),
    }

    if !options.hide {
        display_image(&image).await;
    }
}
