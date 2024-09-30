use macroquad::prelude::*;

struct Tuple {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Tuple {
    fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    fn is_point(&self) -> bool {
        self.w == 1.0
    }

    fn as_color(&self) -> Color {
        Color {
            r: self.x,
            g: self.y,
            b: self.z,
            a: self.w,
        }
    }
}

fn tuple(x: f32, y: f32, z: f32, w: f32) -> Tuple {
    Tuple { x, y, z, w }
}

#[cfg(test)]
mod test_tuple {
    use super::*;

    #[test]
    fn a_tupe_with_w1_0_is_a_point() {
        let a = tuple(4.3, -4.2, 3.1, 1.0);
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 1.0);
        assert_eq!(a.is_point(), true);
        assert_eq!(a.is_vector(), false);
    }
}

#[macroquad::main("Macroquad Ray Tracer")]
async fn main() {
    let width = 80;
    let height = 40;
    let mut image = Image::gen_image_color(width, height, WHITE);
    image.set_pixel(40, 20, tuple(1.0, 0.5, 0.5, 1.0).as_color());
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);

    loop {
        set_default_camera();
        clear_background(PURPLE);
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
