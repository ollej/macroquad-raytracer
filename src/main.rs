use macroquad_raytracer::prelude::*;

use clap::Parser;
use rayon::prelude::*;
use std::f64::consts::PI;
use std::time::Instant;

fn generate_clock(canvas_size: usize) -> Result<Canvas, String> {
    let half_width = canvas_size as Float / 2.0;
    let mut canvas = canvas((half_width * 2.0) as usize, (half_width * 2.0) as usize);
    let origin = point(0.0, 0.0, 0.0);
    let twelve = point(0.0, 0.0, 1.0);
    let red = color(1.0, 0.0, 0.0);
    let radius = half_width * 2.0 * (3.0 / 8.0);

    for h in 0..12 {
        let r = rotation_y(h as Float * (PI / 6.0));
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
    let pixel_size = wall_size / canvas_size as Float;
    let half = wall_size / 2.;
    let mut canvas = canvas(canvas_size, canvas_size);
    let color = color(1.0, 0.0, 0.0);
    let shape = sphere();

    for y in 0..canvas_size {
        let world_y = half - pixel_size * y as Float;
        for x in 0..canvas_size {
            let world_x = -half + pixel_size * x as Float;
            let position = point(world_x, world_y, wall_z);
            let r = ray(&ray_origin, &(position - ray_origin).normalize());
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
    let pixel_size = wall_size / canvas_size as Float;
    let half = wall_size / 2.;
    let mut canvas = canvas(canvas_size, canvas_size);

    let mut sphere = sphere();
    sphere.material.color = color(1.0, 0.2, 1.0);

    let light_position = point(-10., 10., -10.);
    let light_color = WHITE;
    let light = point_light(&light_position, &light_color);

    for y in 0..canvas_size {
        let world_y = half - pixel_size * y as Float;
        for x in 0..canvas_size {
            let world_x = -half + pixel_size * x as Float;
            let position = point(world_x, world_y, wall_z);
            let r = ray(&ray_origin, &(position - ray_origin).normalize());
            let xs = sphere.intersect(&r)?;
            if let Some(hit) = xs.hit() {
                let point = r.position(hit.t);
                let normal = hit.object.normal_at(&point)?;
                let eye = -r.direction;
                let color = hit.object.material.lighting(
                    &hit.object,
                    &light,
                    &point,
                    &eye,
                    &normal,
                    false,
                )?;

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
    let pixel_size = wall_size / canvas_size as Float;
    let half = wall_size / 2.;
    let mut canvas = canvas(canvas_size, canvas_size);

    let mut sphere = sphere();
    sphere.material.color = color(1.0, 0.2, 1.0);

    let light_position = point(-10., 10., -10.);
    let light_color = WHITE;
    let light = point_light(&light_position, &light_color);

    (0..canvas_size)
        .into_par_iter()
        .flat_map(|y| {
            let world_y = half - pixel_size * y as Float;
            let light = light.clone();
            (0..canvas_size).into_par_iter().map(move |x| {
                let world_x = -half + pixel_size * x as Float;
                let position = point(world_x, world_y, wall_z);
                let r = ray(&ray_origin, &(position - ray_origin).normalize());
                let xs = sphere.intersect(&r)?;
                if let Some(hit) = xs.hit() {
                    let point = r.position(hit.t);
                    let normal = hit.object.normal_at(&point)?;
                    let eye = -r.direction;
                    let color = hit.object.material.lighting(
                        &hit.object,
                        &light,
                        &point,
                        &eye,
                        &normal,
                        false,
                    )?;

                    Ok::<(usize, usize, Color), String>((x, y, color))
                } else {
                    Ok((x, y, BLACK))
                }
            })
        })
        .collect::<Result<Vec<(usize, usize, Color)>, String>>()?
        .iter()
        .for_each(|(x, y, color)| canvas.write_pixel(*x, *y, color));

    Ok(canvas)
}

fn generate_scene(canvas_size: usize) -> Result<Canvas, String> {
    let (camera, mut world) = setup_scene(canvas_size);

    let wall_material = Material {
        color: color(1.0, 0.9, 0.9),
        specular: 0.0,
        ..Default::default()
    };

    let floor = Object::new_sphere(scaling(10.0, 0.01, 10.0), wall_material);

    let left_wall = Object::new_sphere(
        translation(0.0, 0.0, 5.0)
            * rotation_y(-PI / 4.0)
            * rotation_x(PI / 2.0)
            * scaling(10.0, 0.01, 10.0),
        wall_material,
    );

    let right_wall = Object::new_sphere(
        translation(0.0, 0.0, 5.0)
            * rotation_y(PI / 4.0)
            * rotation_x(PI / 2.0)
            * scaling(10.0, 0.01, 10.0),
        wall_material,
    );

    let middle = build_sphere(1.0, color(0.1, 1.0, 0.5), translation(-0.5, 1.0, 0.5), None);
    let right = build_sphere(0.5, color(0.5, 1.0, 0.1), translation(1.5, 0.5, -0.5), None);
    let left = build_sphere(
        0.33,
        color(1.0, 0.8, 0.1),
        translation(-1.5, 0.33, -0.75),
        None,
    );

    world
        .objects
        .extend(vec![floor, left_wall, right_wall, middle, right, left]);

    camera.render(&world)
}

fn generate_scene_plane(canvas_size: usize) -> Result<Canvas, String> {
    let (camera, mut world) = setup_scene(canvas_size);

    world.objects.append(&mut build_plane_walls());

    world.objects.push(build_sphere(
        1.0,
        color(0.1, 1.0, 0.5),
        translation(-0.5, 1.0, 0.5),
        None,
    ));
    world.objects.push(build_sphere(
        0.5,
        color(0.5, 1.0, 0.1),
        translation(1.5, 0.5, -0.5) * scaling(0.5, 0.5, 0.5),
        None,
    ));
    world.objects.push(build_sphere(
        0.33,
        color(1.0, 0.8, 0.1),
        translation(-1.5, 0.33, -0.75) * scaling(0.33, 0.33, 0.33),
        None,
    ));

    camera.render(&world)
}

fn generate_scene_pattern(canvas_size: usize) -> Result<Canvas, String> {
    let (camera, mut world) = setup_scene(canvas_size);

    let floor = build_floor_plane();

    world.objects.push(floor);

    let mut wall_material = floor.material.clone();
    wall_material.set_pattern(ring_pattern(&color(1.0, 0.9, 0.9), &color(0.4, 0.4, 0.5)));
    world.objects.push(Object::new_plane(
        translation(0.0, 0.0, 2.5) * rotation_x(PI / 2.0),
        wall_material,
    ));

    let mut pattern = gradient_pattern(&color(1.0, 0.0, 0.0), &color(0.0, 1.0, 0.0));
    pattern.set_transform(rotation_x(-PI / 4.0) * rotation_z(-PI / 4.0) * scaling(0.6, 0.6, 0.6));
    world.objects.push(Object::new_sphere(
        translation(-0.5, 1.0, 0.5),
        Material {
            color: WHITE,
            diffuse: 0.7,
            specular: 0.3,
            pattern: Some(pattern),
            ..Default::default()
        },
    ));
    let mut sphere_pattern = stripe_pattern(&color(0.0, 0.0, 1.0), &color(0.0, 1.0, 1.0));
    sphere_pattern
        .set_transform(rotation_z(PI / 4.0) * rotation_y(PI / 4.0) * scaling(0.4, 0.4, 0.4));
    world.objects.push(build_sphere(
        0.5,
        color(0.5, 1.0, 0.1),
        translation(1.5, 0.5, -0.5) * scaling(0.5, 0.5, 0.5),
        Some(sphere_pattern),
    ));
    let mut stripe_pattern = stripe_pattern(&color(0.0, 1.0, 0.0), &color(1.0, 1.0, 0.0));
    stripe_pattern.set_transform(rotation_x(PI / 4.0) * rotation_z(PI / 4.0));
    world.objects.push(build_sphere(
        0.33,
        color(1.0, 0.8, 0.1),
        translation(-1.5, 0.33, -0.75) * scaling(0.33, 0.33, 0.33),
        Some(stripe_pattern),
    ));

    camera.render(&world)
}

fn generate_scene_reflection(canvas_size: usize) -> Result<Canvas, String> {
    let (camera, mut world) = setup_scene(canvas_size);

    world.objects.push(build_floor_plane());

    world.objects.push(Object::new_sphere(
        translation(-0.5, 1.0, 0.5),
        Material {
            color: color(0.1, 0.1, 0.1),
            ambient: 0.2,
            diffuse: 0.4,
            specular: 0.3,
            reflective: 0.8,
            pattern: None,
            ..Default::default()
        },
    ));
    world.objects.push(Object::new_sphere(
        translation(1.3, 1.0, 1.5) * scaling(0.8, 0.8, 0.8),
        Material {
            color: color(0.1, 0.7, 0.2),
            diffuse: 0.8,
            specular: 0.3,
            ..Default::default()
        },
    ));
    world.objects.push(Object::new_sphere(
        translation(1.5, 0.5, -0.5) * scaling(0.5, 0.5, 0.5),
        Material {
            color: color(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.1,
            specular: 1.0,
            shininess: 300.0,
            transparency: 0.9,
            reflective: 0.9,
            refractive_index: 0.8,
            ..Default::default()
        },
    ));
    world.objects.push(build_sphere(
        0.8,
        color(1.0, 0.8, 0.1),
        translation(-1.5, 0.33, -0.75) * scaling(0.33, 0.33, 0.33),
        None,
    ));

    camera.render(&world)
}

fn setup_scene(canvas_size: usize) -> (Camera, World) {
    let light_source = point_light(&point(-10.0, 10.0, -10.0), &color(1.0, 1.0, 1.0));
    let world = World {
        objects: vec![],
        light: Some(light_source),
    };

    let mut camera = camera(canvas_size, canvas_size / 2, PI / 3.0, MAX_REFLECTIVE_DEPTH);
    camera.transform = view_transform(
        &point(0.0, 1.5, -5.0),
        &point(0.0, 1.0, 0.0),
        &vector(0.0, 1.0, 0.0),
    );

    (camera, world)
}

fn build_plane_walls() -> Vec<Object> {
    let floor_material = Material {
        color: color(1.0, 0.9, 0.9),
        specular: 0.0,
        ..Default::default()
    };

    let floor = Object::new_plane(IDENTITY_MATRIX, floor_material);
    let wall = Object::new_plane(
        translation(0.0, 0.0, 2.5) * rotation_x(PI / 2.0),
        floor_material,
    );

    vec![floor, wall]
}

fn build_floor_plane() -> Object {
    let floor_material = Material {
        color: WHITE,
        specular: 0.0,
        reflective: 0.5,
        pattern: Some(checkers_pattern(
            &color(1.0, 0.9, 0.9),
            &color(0.4, 0.4, 0.5),
        )),
        ..Default::default()
    };
    Object::new_plane(IDENTITY_MATRIX, floor_material)
}

fn build_sphere(
    scale: Float,
    color: Color,
    translation: Matrix,
    pattern: Option<Pattern>,
) -> Object {
    Object::new_sphere(
        translation * scaling(scale, scale, scale),
        Material {
            color,
            diffuse: 0.7,
            specular: 0.3,
            pattern,
            ..Default::default()
        },
    )
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
        Image::Scene => generate_scene(options.size)?,
        Image::ScenePlane => generate_scene_plane(options.size)?,
        Image::ScenePattern => generate_scene_pattern(options.size)?,
        Image::SceneReflection => generate_scene_reflection(options.size)?,
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
