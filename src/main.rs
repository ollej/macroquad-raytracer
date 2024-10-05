use macroquad_raytracer::prelude::*;

use clap::Parser;

#[macroquad::main(window_conf())]
async fn main() {
    let options = AppOptions::parse();

    let c = generate_trajectory();
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
