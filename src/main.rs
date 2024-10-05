use macroquad::prelude::*;

use macroquad_raytracer::prelude::*;

#[macroquad::main("Macroquad Ray Tracer")]
async fn main() {
    let c = generate_trajectory();
    let image = c.as_image();

    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);

    loop {
        set_default_camera();
        clear_background(BLACK);
        draw_texture_ex(
            &texture,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );
        next_frame().await
    }
}
