use crate::prelude::*;

use macroquad::{color::BLACK, texture::Image};
use std::{fs::File, io::Write, path::PathBuf};

pub fn canvas(width: usize, height: usize) -> Canvas {
    Canvas::new(width, height)
}

pub fn pixel_at(canvas: &Canvas, x: usize, y: usize) -> Color {
    canvas.pixel_at(x, y)
}

#[derive(PartialEq, Debug, Clone)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        let pixels = vec![color(0.0, 0.0, 0.0); width * height];
        Self {
            width,
            height,
            pixels,
        }
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: &Color) {
        self.pixels[y * self.width + x] = color.to_owned();
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        self.pixels[y * self.width + x]
    }

    pub fn fill(&mut self, color: &Color) {
        for i in 0..self.width {
            for j in 0..self.height {
                self.write_pixel(i, j, color);
            }
        }
    }

    pub fn as_ppm(&self) -> String {
        let width = self.width;
        let height = self.height;
        let mut output = String::new();
        output.push_str(&format!(
            "P3\n\
            {width} {height}\n\
            255\n"
        ));

        let mut line = String::new();
        for (index, c) in self
            .pixels
            .iter()
            .flat_map(|p| p.as_byte_strings().to_vec())
            .enumerate()
        {
            if line.len() + c.len() >= 70 {
                output.push_str(&line);
                output.push_str("\n");
                line = c;
            } else if (index + 1) % (self.width * 3) == 0 {
                line.push_str(" ");
                line.push_str(&c);
                output.push_str(&line);
                output.push_str("\n");
                line = String::new()
            } else {
                if index % (self.width * 3) != 0 {
                    line.push_str(" ");
                }
                line.push_str(&c);
            }
        }
        output.push_str(&line);
        output
    }

    pub fn as_image(&self) -> Image {
        let mut image = Image::gen_image_color(self.width as u16, self.height as u16, BLACK);
        for x in 0..self.width {
            for y in 0..self.height {
                let p = self.pixels[y * self.width + x];
                image.set_pixel(x as u32, (self.height - y - 1) as u32, p.as_color());
            }
        }
        image
    }

    pub fn save_ppm(&self, path: &PathBuf) {
        let mut output = File::create(path).unwrap();
        write!(output, "{}", self.as_ppm()).unwrap();
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
