use macroquad_raytracer::prelude::*;

use clap::Parser;
use rayon::prelude::*;
use std::f32::consts::PI;
use std::time::Instant;

fn generate_clock(canvas_size: usize) -> Result<Canvas, String> {
    let half_width = canvas_size as f32 / 2.0;
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

fn generate_circle(canvas_size: usize) -> Result<Canvas, String> {
    let ray_origin = point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let pixel_size = wall_size / canvas_size as f32;
    let half = wall_size / 2.;
    let mut canvas = canvas(canvas_size, canvas_size);
    let color = color(1.0, 0.0, 0.0);
    let shape = sphere();

    for y in 0..canvas_size {
        let world_y = half - pixel_size * y as f32;
        for x in 0..canvas_size {
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

fn generate_sphere(canvas_size: usize) -> Result<Canvas, String> {
    let ray_origin = point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let pixel_size = wall_size / canvas_size as f32;
    let half = wall_size / 2.;
    let mut canvas = canvas(canvas_size, canvas_size);

    let mut sphere = sphere();
    sphere.material.color = color(1.0, 0.2, 1.0);

    let light_position = point(-10., 10., -10.);
    let light_color = WHITE;
    let light = point_light(light_position, light_color);

    for y in 0..canvas_size {
        let world_y = half - pixel_size * y as f32;
        for x in 0..canvas_size {
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

fn generate_sphere_rayon(canvas_size: usize) -> Result<Canvas, String> {
    let ray_origin = point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let pixel_size = wall_size / canvas_size as f32;
    let half = wall_size / 2.;
    let mut canvas = canvas(canvas_size, canvas_size);

    let mut sphere = sphere();
    sphere.material.color = color(1.0, 0.2, 1.0);

    let light_position = point(-10., 10., -10.);
    let light_color = WHITE;
    let light = point_light(light_position, light_color);

    let pixels: Vec<(usize, usize, Color)> = (0..canvas_size)
        .into_par_iter()
        .flat_map(|y| {
            let world_y = half - pixel_size * y as f32;
            let light = light.clone();
            (0..canvas_size).into_par_iter().map(move |x| {
                let world_x = -half + pixel_size * x as f32;
                let position = point(world_x, world_y, wall_z);
                let r = ray(ray_origin, (position - ray_origin).normalize());
                let xs = sphere.intersect(&r)?;
                if let Some(hit) = xs.hit() {
                    let point = r.position(hit.t);
                    let normal = hit.object.normal_at(&point)?;
                    let eye = -r.direction;
                    let color = hit.object.material.lighting(&light, &point, &eye, &normal);

                    Ok::<(usize, usize, Color), String>((x, y, color))
                } else {
                    Ok((x, y, BLACK))
                }
            })
        })
        .flatten()
        .collect();

    for (x, y, color) in pixels {
        canvas.write_pixel(x, y, &color);
    }

    //canvas.write_pixel(x, y, &color);

    Ok(canvas)
}

#[macroquad::main(window_conf())]
async fn main() -> Result<(), String> {
    let options = AppOptions::parse();

    let before = Instant::now();
    let c = match options.image {
        Image::Clock => generate_clock(options.size)?,
        Image::Circle => generate_circle(options.size)?,
        Image::Sphere => generate_sphere(options.size)?,
        Image::SphereRayon => generate_sphere_rayon(options.size)?,
    };
    if options.time {
        let elapsed = before.elapsed();
        println!(
            "Elapsed time: {:.3?}s {:.2?}millis {:.2?}micros {:.2?}nanos",
            elapsed.as_secs_f32(),
            elapsed.subsec_millis(),
            elapsed.subsec_micros(),
            elapsed.subsec_nanos()
        );
    }

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
