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
            let sphere = sphere.clone();
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

    world.objects.push(floor.clone());

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

    // Mirror sphere
    world.objects.push(Object::new_sphere(
        translation(-0.5, 1.0, 0.5),
        Material {
            color: color(0.1, 0.1, 0.1),
            ambient: 0.2,
            diffuse: 0.4,
            specular: 0.3,
            reflective: 0.8,
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
    // Transparent sphere
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

fn generate_scene_cube(canvas_size: usize) -> Result<Canvas, String> {
    let (camera, mut world) = setup_scene(canvas_size);

    // Walls
    let mut wall_pattern = stripe_pattern(&color(0.2, 0.8, 0.8), &color(0.8, 0.8, 0.4));
    wall_pattern.set_transform(rotation_z(PI / 2.0) * scaling(0.1, 0.1, 0.1));
    world.objects.push(Object::new_cube(
        //* rotation_z(-PI / 2.0)
        //* rotation_y(-PI / 2.0)
        rotation_y(-PI / 4.0) * translation(-12.5, -12.5, -12.5) * scaling(25.0, 25.0, 25.0), //, //rotation_y(PI / 2.0), // * translation(-15.0, -15.0, -15.0) * scaling(30.0, 30.0, 30.0)
        Material {
            color: color(1.0, 0.0, 0.0),
            pattern: Some(wall_pattern),
            ..Default::default()
        },
    ));

    // Floor
    let mut floor_pattern = checkers_pattern(&color(1.0, 0.9, 0.9), &color(0.4, 0.4, 0.5));
    floor_pattern.set_transform(scaling(0.01, 0.01, 0.01));
    world.objects.push(Object::new_cube(
        translation(-50.0, 28.0, -50.0) * scaling(100.0, 30.0, 100.0),
        Material {
            color: color(1.0, 0.0, 0.0),
            specular: 0.0,
            reflective: 0.1,
            pattern: Some(floor_pattern),
            ..Default::default()
        },
    ));

    // Table
    world.objects.push(Object::new_cube(
        translation(-0.25, -0.25, 5.0) * scaling(2.0, 0.1, 1.25),
        Material {
            color: color(1.0, 0.0, 0.0),
            ambient: 0.2,
            diffuse: 0.8,
            ..Default::default()
        },
    ));
    let leg_material = Material {
        color: color(0.54, 0.27, 0.21),
        ..Default::default()
    };
    world.objects.push(Object::new_cube(
        translation(-2.1, -1.2, 4.8) * scaling(0.1, 1.0, 0.1),
        leg_material,
    ));
    world.objects.push(Object::new_cube(
        translation(-2.1, -1.2, 6.0) * scaling(0.1, 1.0, 0.1),
        leg_material,
    ));
    world.objects.push(Object::new_cube(
        translation(1.6, -1.2, 4.8) * scaling(0.1, 1.0, 0.1),
        leg_material,
    ));
    world.objects.push(Object::new_cube(
        translation(1.6, -1.2, 6.0) * scaling(0.1, 1.0, 0.1),
        leg_material,
    ));

    // Box on table
    world.objects.push(Object::new_cube(
        translation(-0.1, -0.1, 5.0) * scaling(0.2, 0.1, 0.2),
        Material {
            color: color(0.65, 0.43, 0.58),
            ambient: 0.2,
            diffuse: 0.8,
            ..Default::default()
        },
    ));

    // Diamond box
    world.objects.push(Object::new_cube(
        translation(-5.0, -1.0, 6.5)
            * rotation_z(PI / 4.0)
            * rotation_x(PI / 4.0)
            * scaling(0.6, 0.6, 0.6),
        Material {
            color: color(0.1, 0.1, 0.1),
            ambient: 0.1,
            diffuse: 0.1,
            specular: 1.0,
            shininess: 300.0,
            transparency: 0.9,
            reflective: 0.6,
            refractive_index: 2.417, // Diamond
            ..Default::default()
        },
    ));

    // Mirror box
    world.objects.push(Object::new_cube(
        translation(3.5, -1.0, 10.0) * rotation_y(PI / 4.8) * scaling(1.2, 6.0, 0.1),
        Material {
            color: color(0.1, 0.1, 0.1),
            ambient: 0.2,
            diffuse: 0.6,
            specular: 0.3,
            reflective: 0.8,
            ..Default::default()
        },
    ));

    // Floor boxes
    world.objects.push(Object::new_cube(
        translation(4.5, -1.5, 6.0) * scaling(0.5, 0.5, 0.5),
        Material {
            color: color(0.6, 0.3, 0.8),
            ..Default::default()
        },
    ));
    world.objects.push(Object::new_cube(
        translation(4.3, -0.6, 5.8) * rotation_y(PI / 4.0) * scaling(0.3, 0.3, 0.3),
        Material {
            color: color(0.8, 0.6, 0.3),
            ..Default::default()
        },
    ));
    world.objects.push(Object::new_cube(
        translation(3.5, -1.65, 5.5) * rotation_z(PI / 6.0) * scaling(0.2, 0.2, 0.2),
        Material {
            color: color(0.6, 0.8, 0.3),
            ..Default::default()
        },
    ));

    camera.render(&world)
}

fn generate_scene_cylinder(canvas_size: usize) -> Result<Canvas, String> {
    let (camera, mut world) = setup_scene(canvas_size);

    world.objects.push(build_floor_plane());

    world.objects.push(Object::new_cylinder(
        -12.0,
        12.0,
        true,
        translation(0.0, 1.0, 0.0) * scaling(0.1, 0.1, 0.1),
        Material {
            color: color(0.1, 0.1, 0.1),
            shininess: 250.0,
            transparency: 0.6,
            reflective: 0.9,
            refractive_index: 1.52, // Glass
            ..Default::default()
        },
    ));
    world.objects.push(Object::new_cylinder(
        -12.0,
        12.0,
        true,
        translation(0.0, 1.0, 0.0) * rotation_z(PI / 2.0) * scaling(0.1, 0.1, 0.1),
        colored_material(0.07, 0.50, 0.53),
    ));
    world.objects.push(Object::new_cylinder(
        -12.0,
        12.0,
        true,
        translation(0.0, 1.0, 0.0) * rotation_z(PI / 3.0) * scaling(0.1, 0.1, 0.1),
        colored_material(0.33, 0.27, 0.40),
    ));
    world.objects.push(Object::new_cylinder(
        -12.0,
        12.0,
        true,
        translation(0.0, 1.0, 0.0) * rotation_z(PI / 1.5) * scaling(0.1, 0.1, 0.1),
        colored_material(0.80, 0.46, 0.45),
    ));
    world.objects.push(Object::new_cylinder(
        -12.0,
        12.0,
        true,
        translation(0.0, 1.0, 0.0)
            * rotation_y(PI / 4.0)
            * rotation_x(PI / 2.0)
            * scaling(0.1, 0.1, 0.1),
        colored_material(0.93, 0.71, 0.38),
    ));
    world.objects.push(Object::new_cylinder(
        -12.0,
        12.0,
        true,
        translation(0.0, 1.0, 0.0)
            * rotation_y(-PI / 4.0)
            * rotation_x(PI / 2.0)
            * scaling(0.1, 0.1, 0.1),
        colored_material(0.86, 0.53, 0.40),
    ));

    camera.render(&world)
}

fn generate_scene_cone(canvas_size: usize) -> Result<Canvas, String> {
    let (camera, mut world) = setup_scene(canvas_size);

    world.objects.push(build_floor_plane());

    world.objects.push(Object::new_sphere(
        translation(0.0, 2.2, 1.0) * scaling(0.4, 0.4, 0.4),
        Material {
            color: color(1.0, 0.77, 0.85),
            diffuse: 0.9,
            specular: 0.3,
            ..Default::default()
        },
    ));
    world.objects.push(Object::new_sphere(
        translation(0.3, 1.8, 1.0) * scaling(0.4, 0.4, 0.4),
        Material {
            color: color(0.76, 0.95, 0.82),
            diffuse: 0.9,
            specular: 0.3,
            ..Default::default()
        },
    ));
    world.objects.push(Object::new_sphere(
        translation(-0.3, 1.8, 1.0) * scaling(0.4, 0.4, 0.4),
        Material {
            color: color(0.99, 0.96, 0.79),
            diffuse: 0.9,
            specular: 0.3,
            ..Default::default()
        },
    ));
    world.objects.push(unit_cone_upsidedown(
        false,
        translation(0.0, 0.0, 1.0) * scaling(0.5, 1.5, 0.4),
        Material {
            color: color(1.0, 0.80, 0.52),
            diffuse: 0.7,
            specular: 0.3,
            ..Default::default()
        },
    ));

    camera.render(&world)
}

fn generate_scene_hexagon(canvas_size: usize) -> Result<Canvas, String> {
    let (camera, mut world) = setup_scene(canvas_size);

    world.objects.push(build_floor_plane());

    let mut hex = hexagon();
    hex.set_transform(translation(0.0, 1.0, 0.0) * rotation_x(-PI / 6.0));
    world.objects.push(hex);

    camera.render(&world)
}

fn generate_scene_grouped_spheres(canvas_size: usize) -> Result<Canvas, String> {
    let (camera, mut world) = setup_scene(canvas_size);

    let plane = Object::new_plane(
        translation(0.0, 0.0, 20.0) * rotation_x(PI / 2.0),
        colored_material(1.0, 1.0, 1.0),
    );
    world.objects.push(plane);

    let all_spheres = &mut empty_group();
    all_spheres.set_transform(translation(-1.0, 0.0, 0.0) * rotation_y(PI / 6.0));
    for i in 1..=2 {
        for j in 1..=2 {
            for k in 1..=2 {
                let g = &mut empty_group();
                g.set_transform(translation(
                    i as Float * 1.5 - 1.75,
                    j as Float * 1.5 - 1.75,
                    k as Float * 1.5 - 1.0,
                ));
                for z in 0..5 {
                    for y in 0..5 {
                        for x in 0..5 {
                            let s = &mut sphere();
                            s.set_material(&colored_material(
                                i as Float * x as Float / 10.0,
                                y as Float * j as Float / 10.0,
                                z as Float * k as Float / 10.0,
                            ));
                            s.set_transform(
                                translation(
                                    x as Float * 0.3 - 1.0,
                                    y as Float * 0.3 + 0.2,
                                    z as Float * 0.3,
                                ) * scaling(0.1, 0.1, 0.1),
                            );
                            g.add_child(s);
                        }
                    }
                }
                all_spheres.add_child(g);
            }
        }
    }
    world.objects.push(all_spheres.to_owned());

    camera.render(&world)
}

fn hexagon_corner() -> Object {
    let mut corner = sphere();
    corner.set_material(&colored_material(1.0, 0.0, 0.0));
    corner.set_transform(translation(0.0, 0.0, -1.0) * scaling(0.25, 0.25, 0.25));
    corner
}

fn hexagon_edge() -> Object {
    let mut edge = cylinder(0.0, 1.0, true);
    edge.set_material(&colored_material(1.0, 0.0, 0.0));
    edge.set_transform(
        translation(0.0, 0.0, -1.0)
            * rotation_y(-PI / 6.0)
            * rotation_z(-PI / 2.0)
            * scaling(0.25, 1.0, 0.25),
    );
    edge
}

fn hexagon_side() -> Object {
    let mut side = empty_group();
    side.add_child(&mut hexagon_corner());
    side.add_child(&mut hexagon_edge());
    side
}

fn hexagon() -> Object {
    let mut hex = empty_group();
    for n in 0..=5 {
        let side = &mut hexagon_side();
        side.set_transform(rotation_y(n as Float * PI / 3.0));
        hex.add_child(side);
    }
    hex
}

fn setup_scene(canvas_size: usize) -> (Camera, World) {
    let light_source = point_light(&point(-0.0, 10.0, -10.0), &color(1.0, 1.0, 1.0));
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

fn colored_material(r: Float, g: Float, b: Float) -> Material {
    Material {
        color: color(r, g, b),
        ..Default::default()
    }
}

fn build_cube(color: Color, translation: Matrix) -> Object {
    Object::new_sphere(
        translation,
        Material {
            color,
            diffuse: 0.7,
            specular: 0.3,
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
        Image::SceneCube => generate_scene_cube(options.size)?,
        Image::SceneCylinder => generate_scene_cylinder(options.size)?,
        Image::SceneCone => generate_scene_cone(options.size)?,
        Image::SceneHexagon => generate_scene_hexagon(options.size)?,
        Image::SceneGroupedSpheres => generate_scene_grouped_spheres(options.size)?,
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
