use crate::prelude::*;

use macroquad::{color::BLACK, texture::Image};

pub fn canvas(width: usize, height: usize) -> Canvas {
    Canvas::new(width, height)
}

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
                if p.red() > 0.0 {
                    println!("x: {}, y: {}, p: {:?}", x, y, p);
                }
                image.set_pixel(x as u32, y as u32, p.as_color());
            }
        }
        image
    }
}
