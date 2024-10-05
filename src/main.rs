use macroquad::prelude::*;

use macroquad_raytracer::prelude::*;

#[cfg(test)]
mod test_chapter_2_colors {
    use super::*;

    #[test]
    fn colors_are_red_green_blue_tuples() {
        let c = color(-0.5, 0.4, 1.7);
        assert_eq!(c.red(), -0.5);
        assert_eq!(c.green(), 0.4);
        assert_eq!(c.blue(), 1.7);
    }

    #[test]
    fn adding_colors() {
        let c1 = color(0.9, 0.6, 0.75);
        let c2 = color(0.7, 0.1, 0.25);
        assert_eq!(c1 + c2, color(1.6, 0.7, 1.0));
    }

    #[test]
    fn subtracting_colors() {
        let c1 = color(0.9, 0.6, 0.75);
        let c2 = color(0.7, 0.1, 0.25);
        assert_eq!(c1 - c2, color(0.2, 0.5, 0.5));
    }

    #[test]
    fn multiplying_a_color_by_a_scalar() {
        let c = color(0.2, 0.3, 0.4);
        assert_eq!(c * 2.0, color(0.4, 0.6, 0.8));
    }

    #[test]
    fn multiplying_colors() {
        let c1 = color(1.0, 0.2, 0.4);
        let c2 = color(0.9, 1.0, 0.1);
        assert_eq!(c1 * c2, color(0.9, 0.2, 0.04));
    }
}

#[cfg(test)]
mod test_chapter_2_canvas {
    use super::*;

    #[test]
    fn creating_a_canvas() {
        let c = canvas(10, 20);

        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
        for pixel in c.pixels {
            assert_eq!(pixel, color(0.0, 0.0, 0.0));
        }
    }

    #[test]
    fn writing_pixels_to_a_canvas() {
        let mut c = canvas(10, 20);
        let red = color(1.0, 0.0, 0.0);
        c.write_pixel(2, 3, &red);

        assert_eq!(c.pixel_at(2, 3), red);
    }

    #[test]
    fn constructing_the_ppm_header() {
        let c = canvas(5, 3);
        let ppm = c.as_ppm();

        assert_eq!(
            ppm.lines().take(3).collect::<Vec<&str>>(),
            ["P3", "5 3", "255"]
        );
    }

    #[test]
    fn constructing_the_ppm_pixel_data() {
        let mut c = canvas(5, 3);
        let c1 = color(1.5, 0.0, 0.0);
        let c2 = color(0.0, 0.5, 0.0);
        let c3 = color(-0.5, 0.0, 1.0);
        c.write_pixel(0, 0, &c1);
        c.write_pixel(2, 1, &c2);
        c.write_pixel(4, 2, &c3);
        let ppm = c.as_ppm();

        assert_eq!(
            ppm.lines().skip(3).take(3).collect::<Vec<&str>>(),
            [
                "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0",
                "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0",
                "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255"
            ]
        );
    }

    #[test]
    fn splitting_long_lines_in_ppm_files() {
        let mut c = canvas(10, 2);
        c.fill(&color(1.0, 0.8, 0.6));
        let ppm = c.as_ppm();

        assert_eq!(
            ppm.lines().skip(3).take(4).collect::<Vec<&str>>(),
            [
                "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204",
                "153 255 204 153 255 204 153 255 204 153 255 204 153",
                "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204",
                "153 255 204 153 255 204 153 255 204 153 255 204 153"
            ]
        );
    }

    #[test]
    fn ppm_files_are_terminated_by_a_newline_character() {
        let c = canvas(5, 3);
        let ppm = c.as_ppm();

        assert_eq!(ppm.chars().last().unwrap(), '\n');
    }

    #[test]
    fn test_generate_trajectory() {
        use std::fs::File;
        use std::io::Write;

        let c = generate_trajectory();

        let ppm = c.as_ppm();
        let mut output = File::create("image.ppm").unwrap();
        write!(output, "{}", ppm).unwrap();
    }
}

#[macroquad::main("Macroquad Ray Tracer")]
async fn main() {
    //let width = 80;
    //let height = 40;
    //let mut image = Image::gen_image_color(width, height, WHITE);
    //image.set_pixel(40, 20, tuple(1.0, 0.5, 0.5, 1.0).as_color());

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
