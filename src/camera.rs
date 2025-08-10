use crate::{canvas::*, color::*, float::*, matrix::*, ray::*, tuple::*, world::*};

use rayon::prelude::*;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Camera {
    pub hsize: usize,
    pub vsize: usize,
    pub field_of_view: Float,
    pub pixel_size: Float,
    pub half_width: Float,
    pub half_height: Float,
    pub transform: Matrix,
    pub inverse_transform: Matrix,
    pub reflective_depth: usize,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: Float, reflective_depth: usize) -> Self {
        let half_view = (field_of_view / 2.0).tan();
        let aspect = hsize as Float / vsize as Float;
        let (half_width, half_height) = if aspect >= 1.0 {
            (half_view, half_view / aspect)
        } else {
            (half_view * aspect, half_view)
        };
        let pixel_size = (half_width * 2.0) / hsize as Float;
        Camera {
            hsize,
            vsize,
            field_of_view,
            pixel_size,
            half_width,
            half_height,
            transform: IDENTITY_MATRIX,
            inverse_transform: IDENTITY_MATRIX.inverse().unwrap(),
            reflective_depth,
        }
    }

    pub fn set_transform(&mut self, transform: Matrix) -> Result<(), String> {
        self.transform = transform;
        self.inverse_transform = transform.inverse()?;
        Ok(())
    }

    pub fn ray_for_pixel(&self, px: usize, py: usize) -> Ray {
        // The offset from the edge of the canvas to the pixel's center.
        let xoffset = (px as Float + 0.5) * self.pixel_size;
        let yoffset = (py as Float + 0.5) * self.pixel_size;

        // The untransformed coordinates of the pixel in world space.
        // (remember that the camera looks toward -z, so +x is to the *left*.)
        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        // Using the camera matrix, transform the canvas point and the origin,
        // and then compute the ray's direction vector.
        // (remember that the canvas is at z = -1)
        let pixel = self.inverse_transform * point(world_x, world_y, -1.0);
        let origin = self.inverse_transform * point(0.0, 0.0, 0.0);
        let direction = (pixel - origin).normalize();

        ray(&origin, &direction)
    }

    pub fn render(&self, world: &World) -> Canvas {
        let mut image = canvas(self.hsize, self.vsize);

        (0..self.vsize)
            .into_par_iter()
            .flat_map(|y| {
                (0..self.hsize).into_par_iter().map(move |x| {
                    let ray = self.ray_for_pixel(x, y);
                    let color = world.color_at(&ray, self.reflective_depth);

                    (x, y, color)
                })
            })
            .collect::<Vec<(usize, usize, Color)>>()
            .iter()
            .for_each(|(x, y, color)| image.write_pixel(*x, *y, color));

        image
    }
}

pub fn camera(px: usize, py: usize, field_of_view: Float, reflective_depth: usize) -> Camera {
    Camera::new(px, py, field_of_view, reflective_depth)
}

pub fn ray_for_pixel(camera: &Camera, hsize: usize, vsize: usize) -> Ray {
    camera.ray_for_pixel(hsize, vsize)
}

pub fn render(camera: &Camera, world: &World) -> Canvas {
    camera.render(world)
}

#[cfg(test)]
mod test_chapter_7_camera {
    use super::*;

    #[test]
    fn constructing_a_camera() {
        let hsize = 160;
        let vsize = 120;
        let field_of_view = PI / 2.0;
        let c = camera(hsize, vsize, field_of_view, 0);
        assert_eq!(c.hsize, 160);
        assert_eq!(c.vsize, 120);
        assert_eq!(c.field_of_view, PI / 2.0);
        assert_eq!(c.transform, IDENTITY_MATRIX);

        let c2 = Camera::new(hsize, vsize, field_of_view, 0);
        assert_eq!(c, c2);
    }

    #[test]
    fn the_pixel_size_for_a_horizontal_canvas() {
        let c = camera(200, 125, PI / 2.0, 0);
        assert_eq_float!(c.pixel_size, 0.01);
    }

    #[test]
    fn the_pixel_size_for_a_vertical_canvas() {
        let c = camera(125, 200, PI / 2.0, 0);
        assert_eq_float!(c.pixel_size, 0.01);
    }

    #[test]
    fn constructing_a_ray_through_the_center_of_the_canvas() {
        let c = camera(201, 101, PI / 2.0, 0);
        let r = ray_for_pixel(&c, 100, 50);
        assert_eq!(r.origin, point(0.0, 0.0, 0.0));
        assert_eq!(r.direction, vector(0.0, 0.0, -1.0));

        let r2 = c.ray_for_pixel(100, 50);
        assert_eq!(r, r2);
    }

    #[test]
    fn constructing_a_ray_through_a_corner_of_the_canvas() {
        let c = camera(201, 101, PI / 2.0, 0);
        let r = ray_for_pixel(&c, 0, 0);
        assert_eq!(r.origin, point(0.0, 0.0, 0.0));
        assert_eq!(r.direction, vector(0.66519, 0.33259, -0.66851));
    }

    #[test]
    fn constructing_a_ray_when_the_camera_is_transformed() {
        let mut c = camera(201, 101, PI / 2.0, 0);
        c.set_transform(rotation_y(PI / 4.0) * translation(0.0, -2.0, 5.0))
            .unwrap();
        let r = ray_for_pixel(&c, 100, 50);
        assert_eq!(r.origin, point(0.0, 2.0, -5.0));
        assert_eq!(
            r.direction,
            vector(Float::sqrt(2.0) / 2.0, 0.0, -Float::sqrt(2.0) / 2.0)
        );
    }

    #[test]
    fn rendering_a_world_with_a_camera() {
        let w = default_world();
        let mut c = camera(11, 11, PI / 2.0, 0);
        let from = point(0.0, 0.0, -5.0);
        let to = point(0.0, 0.0, 0.0);
        let up = vector(0.0, 1.0, 0.0);
        c.set_transform(view_transform(&from, &to, &up)).unwrap();
        let image = render(&c, &w);
        assert_eq!(pixel_at(&image, 5, 5), color(0.38066, 0.47583, 0.2855));
        assert_eq!(image.pixel_at(5, 5), color(0.38066, 0.47583, 0.2855));

        let image2 = c.render(&w);
        assert_eq!(image, image2);
    }
}
