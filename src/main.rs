use macroquad_raytracer::prelude::*;

#[macroquad::main("Macroquad Ray Tracer")]
async fn main() {
    let c = generate_trajectory();
    let image = c.as_image();

    display_image(&image).await;
}
