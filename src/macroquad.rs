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
    let image_ratio = image.height as f32 / image.width as f32;

    loop {
        #[cfg(not(target_arch = "wasm32"))]
        if is_key_pressed(KeyCode::Q) | is_key_pressed(KeyCode::Escape) {
            break;
        }

        let width = screen_width().min(screen_height() / image_ratio);
        let height = screen_height().min(screen_width() * image_ratio);

        let x = (screen_width() - width) / 2.0;
        let y = (screen_height() - height) / 2.0;

        set_default_camera();
        clear_background(BLACK);
        draw_texture_ex(
            &texture,
            x,
            y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(width, height)),
                ..Default::default()
            },
        );
        next_frame().await
    }
}

pub fn save_png(image: &Image, path: &PathBuf) {
    image.export_png(&path.display().to_string())
}
