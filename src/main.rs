use macroquad_raytracer::prelude::*;

use clap::Parser;
use std::f32::consts::PI;

fn generate_clock() -> Result<Canvas, String> {
    let half_width = 50.0;
    let mut canvas = canvas((half_width * 2.0) as usize, (half_width * 2.0) as usize);
    let origin = point(0.0, 0.0, 0.0);
    let twelve = point(0.0, 0.0, 1.0);
    let red = color(1.0, 0.0, 0.0);
    let radius = half_width * 2.0 * (3.0 / 8.0);

    for h in 0..12 {
        let r = rotation_y(h as f32 * (PI / 6.0));
        let hour = r * twelve;
        let x = ((hour.x + origin.x) * radius + half_width).round() as usize;
        let y = ((hour.z + origin.z) * radius + half_width).round() as usize;

        canvas.write_pixel(x, y, &red);
    }

    Ok(canvas)
}

fn generate_circle() -> Result<Canvas, String> {
    let ray_origin = point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let canvas_pixels = 100;
    let pixel_size = wall_size / canvas_pixels as f32;
    let half = wall_size / 2.;
    let mut canvas = canvas(canvas_pixels, canvas_pixels);
    let color = color(1.0, 0.0, 0.0);
    let shape = sphere();

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * y as f32;
        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * x as f32;
            let position = point(world_x, world_y, wall_z);
            let r = ray(ray_origin, (position - ray_origin).normalize());
            let xs = shape.intersect(&r)?;
            if xs.hit().is_some() {
                canvas.write_pixel(x, y, &color);
            }
        }
    }

    Ok(canvas)
}

fn generate_sphere() -> Result<Canvas, String> {
    let ray_origin = point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let canvas_pixels = 100;
    let pixel_size = wall_size / canvas_pixels as f32;
    let half = wall_size / 2.;
    let mut canvas = canvas(canvas_pixels, canvas_pixels);

    let mut sphere = sphere();
    sphere.material.color = color(1.0, 0.2, 1.0);

    let light_position = point(-10., 10., -10.);
    let light_color = WHITE;
    let light = point_light(light_position, light_color);

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * y as f32;
        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * x as f32;
            let position = point(world_x, world_y, wall_z);
            let r = ray(ray_origin, (position - ray_origin).normalize());
            let xs = sphere.intersect(&r)?;
            if let Some(hit) = xs.hit() {
                let point = r.position(hit.t);
                let normal = hit.object.normal_at(&point)?;
                let eye = -r.direction;
                let color = hit.object.material.lighting(&light, &point, &eye, &normal);

                canvas.write_pixel(x, y, &color);
            }
        }
    }

    Ok(canvas)
}

#[macroquad::main(window_conf())]
async fn main() -> Result<(), String> {
    let options = AppOptions::parse();

    let c = match options.image {
        Image::Clock => generate_clock()?,
        Image::Circle => generate_circle()?,
        Image::Sphere => generate_sphere()?,
    };

    let image = c.as_image();

    match options.format {
        Some(ImageFormat::PNG) => save_png(&image, &options.image_path().unwrap()),
        Some(ImageFormat::PPM) => c.save_ppm(&options.image_path().unwrap()),
        None => (),
    }

    if !options.hide {
        display_image(&image).await;
    }

    Ok(())
}
