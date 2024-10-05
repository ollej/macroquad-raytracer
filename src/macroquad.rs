use macroquad::prelude::*;

use std::path::PathBuf;

pub fn window_conf() -> Conf {
    Conf {
        window_title: "Macroquad Ray Tracer".to_owned(),
        fullscreen: true,
        high_dpi: true,
        ..Default::default()
    }
}

pub async fn display_image(image: &Image) {
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);

    loop {
        #[cfg(not(target_arch = "wasm32"))]
        if is_key_pressed(KeyCode::Q) | is_key_pressed(KeyCode::Escape) {
            break;
        }

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

pub fn save_png(image: &Image, path: &PathBuf) {
    image.export_png(&path.display().to_string())
}
