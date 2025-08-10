use crate::{
    color::*, float::*, intersection::*, light::*, material::*, matrix::*, object::*, ray::*,
    sphere::*, tuple::*,
};

pub fn world() -> World {
    World::new()
}

pub fn default_world() -> World {
    World::default()
}

pub fn intersect_world(world: &World, ray: &Ray) -> Intersections {
    world.intersect(ray)
}

pub fn shade_hit(
    world: &World,
    prepared_computations: &PreparedComputations,
    remaining: usize,
) -> Color {
    world.shade_hit(prepared_computations, remaining)
}

pub fn color_at(world: &World, ray: &Ray, remaining: usize) -> Color {
    world.color_at(ray, remaining)
}

pub fn is_shadowed(world: &World, light_position: &Point, point: &Point) -> bool {
    world.is_shadowed(light_position, point)
}

#[derive(PartialEq, Clone, Debug)]
pub struct World {
    pub objects: Vec<Object>,
    pub lights: Vec<Light>,
}

impl Default for World {
    fn default() -> World {
        let mut s1 = sphere().unwrap();
        let m = Material {
            color: color(0.8, 1.0, 0.6),
            diffuse: 0.7,
            specular: 0.2,
            ..Default::default()
        };
        s1.set_material(&m);
        let mut s2 = sphere().unwrap();
        s2.set_transform(scaling(0.5, 0.5, 0.5)).unwrap();
        World {
            objects: vec![s1, s2],
            lights: vec![point_light(
                &point(-10.0, 10.0, -10.),
                &color(1.0, 1.0, 1.0),
            )],
        }
    }
}

impl World {
    pub fn new() -> Self {
        Self {
            objects: vec![],
            lights: vec![],
        }
    }

    pub fn set_lights(&mut self, lights: Vec<Light>) {
        self.lights = lights;
    }

    pub fn add_light(&mut self, light: &Light) {
        self.lights.push(light.to_owned());
    }

    pub fn contains(&self, object: &Object) -> bool {
        self.objects.contains(object)
    }

    pub fn intersect(&self, ray: &Ray) -> Intersections {
        let mut all_intersections = vec![];
        for obj in self.objects.iter() {
            let mut intersections = obj.intersect(ray).inner().to_owned();
            all_intersections.append(&mut intersections);
        }
        Intersections::new(all_intersections)
    }

    pub fn shade_hit(&self, comps: &PreparedComputations, remaining: usize) -> Color {
        let mut acc_color = BLACK;
        for light in self.lights.iter() {
            let light_intensity = light.intensity_at(&comps.over_point, self);
            let surface_color = comps.object.lighting(
                light,
                &comps.over_point,
                &comps.eyev,
                &comps.normalv,
                light_intensity,
            );

            let reflected_color = self.reflected_color(&comps, remaining);
            let refracted_color = self.refracted_color(&comps, remaining);

            if comps.object.is_reflective() && !comps.object.is_transparent() {
                let reflectance = comps.schlick();
                acc_color += surface_color
                    + reflected_color * reflectance
                    + refracted_color * (1.0 - reflectance);
            } else {
                acc_color += surface_color + reflected_color + refracted_color
            }
        }
        acc_color
    }

    pub fn color_at(&self, ray: &Ray, remaining: usize) -> Color {
        let intersections = self.intersect(ray);
        match intersections.hit() {
            Some(hit) => {
                let prepared_computations = hit.prepare_computations(ray, &intersections);
                self.shade_hit(&prepared_computations, remaining)
            }
            None => BLACK,
        }
    }

    pub fn is_shadowed(&self, light_position: &Point, point: &Point) -> bool {
        let v = light_position - point;
        let distance = v.magnitude();
        let direction = v.normalize();
        let r = ray(point, &direction);
        self.intersect(&r)
            .hit()
            .map(|hit| hit.object.has_shadow() && hit.t < distance)
            .unwrap_or(false)
    }

    fn reflected_color(&self, comps: &PreparedComputations, remaining: usize) -> Color {
        if remaining < 1 || !comps.object.is_reflective() {
            BLACK
        } else {
            let reflect_ray = ray(&comps.over_point, &comps.reflectv);
            let c = self.color_at(&reflect_ray, remaining - 1);
            c * comps.object.material.reflective
        }
    }

    fn refracted_color(&self, comps: &PreparedComputations, remaining: usize) -> Color {
        if remaining == 0 || comps.object.is_transparent() {
            return BLACK;
        }

        if comps.sin2_t > 1.0 {
            return BLACK;
        }

        // Find cos(theta_t) via trigonometric identity
        let cos_t = Float::sqrt(1.0 - comps.sin2_t);
        // Compute the direction of the refracted ray
        let direction =
            comps.normalv * (comps.n_ratio * comps.cos_i - cos_t) - comps.eyev * comps.n_ratio;
        // Create the refracted ray
        let refract_ray = Ray::new(comps.under_point, direction);
        // Find the color of the refracted ray, making sure to multiply
        // by the transparency value to account for any opacity
        let c = self.color_at(&refract_ray, remaining - 1);
        c * comps.object.material.transparency
    }
}

#[cfg(test)]
mod test_chapter_7_world {
    use super::*;

    #[test]
    fn creating_a_world() {
        let w = world();
        assert_eq!(w.objects, vec![]);
        assert_eq!(w.lights, vec![]);
    }

    #[test]
    fn the_default_world() {
        let light = point_light(&point(-10.0, 10.0, -10.), &color(1.0, 1.0, 1.0));
        let mut s1 = sphere().unwrap();
        let m = Material::new(
            color(0.8, 1.0, 0.6),
            0.1,
            0.7,
            0.2,
            200.0,
            0.0,
            0.0,
            1.0,
            None,
        );
        s1.set_material(&m);
        let mut s2 = sphere().unwrap();
        s2.set_transform(scaling(0.5, 0.5, 0.5)).unwrap();
        let w = default_world();
        assert_eq!(w.lights.first(), Some(&light));
        assert!(w.contains(&s1));
        assert!(w.contains(&s2));

        let w2 = World::default();
        assert_eq!(w, w2);
    }

    #[test]
    fn intersect_a_world_with_a_ray() {
        let w = default_world();
        let r = ray(&point(0.0, 0.0, -5.0), &vector(0.0, 0.0, 1.0));
        let xs = intersect_world(&w, &r);
        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 4.5);
        assert_eq!(xs[2].t, 5.5);
        assert_eq!(xs[3].t, 6.0);
    }

    #[test]
    fn shading_an_intersection() {
        let w = default_world();
        let r = ray(&point(0.0, 0.0, -5.0), &vector(0.0, 0.0, 1.0));
        let shape = w.objects.first().unwrap();
        let i = intersection(4.0, shape.clone());
        let comps = i.prepare_computations(&r, &intersections(vec![i.clone()]));
        let c = shade_hit(&w, &comps, 0);
        assert_eq!(c, color(0.38066, 0.47583, 0.2855));

        let c2 = w.shade_hit(&comps, 0);
        assert_eq!(c, c2);
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let mut w = default_world();
        w.set_lights(vec![point_light(
            &point(0.0, 0.25, 0.0),
            &color(1.0, 1.0, 1.0),
        )]);
        let r = ray(&point(0.0, 0.0, 0.0), &vector(0.0, 0.0, 1.0));
        let shape = w.objects.get(1).unwrap();
        let i = intersection(0.5, shape.clone());
        let comps = i.prepare_computations(&r, &intersections(vec![i.clone()]));
        let c = w.shade_hit(&comps, 0);
        assert_eq!(c, color(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn the_color_when_a_ray_misses() {
        let w = default_world();
        let r = ray(&point(0.0, 0.0, -5.0), &vector(0.0, 1.0, 0.0));
        let c = color_at(&w, &r, 0);
        assert_eq!(c, color(0.0, 0.0, 0.0));

        let c2 = w.color_at(&r, 0);
        assert_eq!(c, c2);
    }

    #[test]
    fn the_color_when_a_ray_hits() {
        let w = default_world();
        let r = ray(&point(0.0, 0.0, -5.0), &vector(0.0, 0.0, 1.0));
        let c = w.color_at(&r, 0);
        assert_eq!(c, color(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn the_color_with_an_intersection_behind_the_ray() {
        let mut w = default_world();
        let mut outer = w.objects[0].clone();
        outer.material.ambient = 1.0;
        w.objects[0] = outer.clone();
        let mut inner = w.objects[1].clone();
        inner.material.ambient = 1.0;
        w.objects[1] = inner.clone();
        let r = ray(&point(0.0, 0.0, 0.75), &vector(0.0, 0.0, -1.0));
        let c = w.color_at(&r, 0);
        assert_eq!(c, inner.material.color);
    }
}

#[cfg(test)]
mod test_chapter_8_shadows {
    use super::*;

    #[test]
    fn there_is_no_shadow_when_nothing_is_collinear_with_point_and_light() {
        let w = default_world();
        let p = point(0.0, 10.0, 0.0);
        let l = w.lights.first().unwrap().position;
        assert_eq!(is_shadowed(&w, &p, &l), false);
        assert_eq!(w.is_shadowed(&p, &l), false);
    }

    #[test]
    fn the_shadow_when_an_object_is_between_the_point_and_the_light() {
        let w = default_world();
        let p = point(10.0, -10.0, 10.0);
        let l = w.lights.first().unwrap().position;
        assert_eq!(w.is_shadowed(&p, &l), true);
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_light() {
        let w = default_world();
        let p = point(-20.0, 20.0, -20.0);
        let l = w.lights.first().unwrap().position;
        assert_eq!(w.is_shadowed(&p, &l), false);
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_point() {
        let w = default_world();
        let p = point(-2.0, 2.0, -2.0);
        let l = w.lights.first().unwrap().position;
        assert_eq!(w.is_shadowed(&p, &l), false);
    }

    #[test]
    fn shade_hit_is_given_an_intersection_in_shadow() {
        let mut w = world();
        w.add_light(&point_light(&point(0.0, 0.0, -10.0), &color(1.0, 1.0, 1.0)));
        let s1 = sphere().unwrap();
        w.objects.push(s1);
        let s2 = Object::new_sphere(translation(0.0, 0.0, 10.0), Material::default()).unwrap();
        w.objects.push(s2.clone());
        let r = ray(&point(0.0, 0.0, 5.0), &vector(0.0, 0.0, 1.0));
        let i = intersection(4.0, s2);
        let comps = prepare_computations(&i, &r, &intersections(vec![i.clone()]));
        let c = w.shade_hit(&comps, 0);
        assert_eq!(c, color(0.1, 0.1, 0.1));
    }
}

#[cfg(test)]
mod test_chapter_11_reflection {
    #![allow(non_snake_case)]

    use super::*;

    use crate::{pattern::test_pattern, plane::*};

    #[test]
    fn the_reflected_color_for_a_nonreflective_material() {
        let mut w = default_world();
        let r = ray(&point(0.0, 0.0, 0.0), &vector(0.0, 0.0, 1.0));
        let mut shape = w.objects[1].clone();
        shape.material.ambient = 1.0;
        w.objects[1] = shape.clone();
        let i = intersection(1.0, shape);
        let comps = i.prepare_computations(&r, &intersections(vec![i.clone()]));
        let c = w.reflected_color(&comps, 0);
        assert_eq!(c, color(0.0, 0.0, 0.0));
    }

    #[test]
    fn the_reflected_color_for_a_reflective_material() {
        let mut w = default_world();
        let mut shape = plane().unwrap();
        shape.material.reflective = 0.5;
        shape.set_transform(translation(0.0, -1.0, 0.0)).unwrap();
        w.objects.push(shape.clone());
        let r = ray(
            &point(0.0, 0.0, -3.0),
            &vector(0.0, -Float::sqrt(2.0) / 2.0, Float::sqrt(2.0) / 2.0),
        );
        let i = intersection(Float::sqrt(2.0), shape);
        let comps = i.prepare_computations(&r, &intersections(vec![i.clone()]));
        let c = w.reflected_color(&comps, 1);
        assert_eq!(c, color(0.1903322, 0.237915, 0.142749));
    }

    #[test]
    fn the_shade_hit_for_a_reflective_material() {
        let mut w = default_world();
        let mut shape = plane().unwrap();
        shape.material.reflective = 0.5;
        shape.set_transform(translation(0.0, -1.0, 0.0)).unwrap();
        w.objects.push(shape.clone());
        let r = ray(
            &point(0.0, 0.0, -3.0),
            &vector(0.0, -Float::sqrt(2.0) / 2.0, Float::sqrt(2.0) / 2.0),
        );
        let i = intersection(Float::sqrt(2.0), shape);
        let comps = i.prepare_computations(&r, &intersections(vec![i.clone()]));
        let c = w.shade_hit(&comps, 1);
        assert_eq!(c, color(0.876757, 0.924340, 0.829174));
    }

    #[test]
    fn color_at_with_mutually_reflective_surfaces() {
        let mut w = world();
        w.add_light(&point_light(&point(0.0, 0.0, 0.0), &color(1.0, 1.0, 1.0)));
        let mut lower = plane().unwrap();
        lower.material.reflective = 1.0;
        lower.set_transform(translation(0.0, -1.0, 0.0)).unwrap();
        w.objects.push(lower);
        let mut upper = plane().unwrap();
        upper.material.reflective = 1.0;
        upper.set_transform(translation(0.0, 1.0, 0.0)).unwrap();
        w.objects.push(upper);
        let r = ray(&point(0.0, 0.0, 0.0), &vector(0.0, 1.0, 0.0));
        let c = w.color_at(&r, 1);
        assert_eq!(c, color(3.8, 3.8, 3.8));
    }

    #[test]
    fn the_reflected_color_at_the_maximum_recursive_depth() {
        let mut w = default_world();
        let mut shape = plane().unwrap();
        shape.material.reflective = 0.5;
        shape.set_transform(translation(0.0, -1.0, 0.0)).unwrap();
        w.objects.push(shape.clone());
        let r = ray(
            &point(0.0, 0.0, -3.0),
            &vector(0.0, -Float::sqrt(2.0) / 2.0, Float::sqrt(2.0) / 2.0),
        );
        let i = intersection(Float::sqrt(2.0), shape);
        let comps = i.prepare_computations(&r, &intersections(vec![i.clone()]));
        let c = w.reflected_color(&comps, 0);
        assert_eq!(c, color(0.0, 0.0, 0.0));
    }

    #[test]
    fn the_refracted_color_with_an_opaque_surface() {
        let w = default_world();
        let shape = w.objects.first().unwrap();
        let r = ray(&point(0.0, 0.0, -5.0), &vector(0.0, 0.0, 1.0));
        let xs = intersections(vec![
            Intersection::new(4.0, shape.clone()),
            Intersection::new(6.0, shape.clone()),
        ]);
        let comps = prepare_computations(&xs[0], &r, &xs);
        let c = w.refracted_color(&comps, 5);
        assert_eq!(c, color(0.0, 0.0, 0.0));
    }

    #[test]
    fn the_refracted_color_at_the_maximum_recursive_depth() {
        let mut w = default_world();
        let mut shape = w.objects[0].clone();
        shape.material.transparency = 1.0;
        shape.material.refractive_index = 1.5;
        w.objects[0] = shape.clone();
        let r = ray(&point(0.0, 0.0, -5.0), &vector(0.0, 0.0, 1.0));
        let xs = intersections(vec![
            Intersection::new(4.0, shape.clone()),
            Intersection::new(6.0, shape),
        ]);
        let comps = prepare_computations(&xs[0], &r, &xs);
        let c = w.refracted_color(&comps, 0);
        assert_eq!(c, color(0.0, 0.0, 0.0));
    }

    #[test]
    fn the_refracted_color_under_total_internal_reflection() {
        let mut w = default_world();
        let mut shape = w.objects[0].clone();
        shape.material.transparency = 1.0;
        shape.material.refractive_index = 1.5;
        w.objects[0] = shape.clone();
        let r = ray(
            &point(0.0, 0.0, Float::sqrt(2.0) / 2.0),
            &vector(0.0, 1.0, 0.0),
        );
        let xs = intersections(vec![
            Intersection::new(-Float::sqrt(2.0) / 2.0, shape.clone()),
            Intersection::new(Float::sqrt(2.0) / 2.0, shape),
        ]);
        // NOTE: this time you're inside the sphere, so you need;
        // to look at the second intersection, xs[1], not xs[0];
        let comps = prepare_computations(&xs[1], &r, &xs);
        let c = w.refracted_color(&comps, 5);
        assert_eq!(c, color(0.0, 0.0, 0.0));
    }

    #[test]
    fn the_refracted_color_with_a_refracted_ray() {
        let mut w = default_world();
        let mut A = w.objects[0].clone();
        A.material.ambient = 1.0;
        A.material.set_pattern(test_pattern());
        w.objects[0] = A.clone();
        let mut B = w.objects[1].clone();
        B.material.transparency = 1.0;
        B.material.refractive_index = 1.5;
        w.objects[1] = B.clone();
        let r = ray(&point(0.0, 0.0, 0.1), &vector(0.0, 1.0, 0.0));
        let xs = intersections(vec![
            Intersection::new(-0.9899, A.clone()),
            Intersection::new(-0.4899, B.clone()),
            Intersection::new(0.4899, B),
            Intersection::new(0.9899, A),
        ]);
        let comps = prepare_computations(&xs[2], &r, &xs);
        let c = w.refracted_color(&comps, 5);
        assert_eq!(c, color(0.0, 0.99888, 0.04725));
    }

    #[test]
    fn shade_hit_with_a_transparent_material() {
        let mut w = default_world();
        let mut floor = plane().unwrap();
        floor.set_transform(translation(0.0, -1.0, 0.0)).unwrap();
        floor.material.transparency = 0.5;
        floor.material.refractive_index = 1.5;
        w.objects.push(floor.clone());
        let mut ball = sphere().unwrap();
        ball.material.color = color(1.0, 0.0, 0.0);
        ball.material.ambient = 0.5;
        ball.set_transform(translation(0.0, -3.5, -0.5)).unwrap();
        w.objects.push(ball);
        let r = ray(
            &point(0.0, 0.0, -3.0),
            &vector(0.0, -Float::sqrt(2.0) / 2.0, Float::sqrt(2.0) / 2.0),
        );
        let xs = intersections(vec![Intersection::new(Float::sqrt(2.0), floor)]);
        let comps = prepare_computations(&xs[0], &r, &xs);
        let c = w.shade_hit(&comps, 5);
        assert_eq!(c, color(0.93642, 0.68642, 0.68642));
    }

    #[test]
    fn shade_hit_with_a_reflective_transparent_material() {
        let mut w = default_world();

        let mut floor = plane().unwrap();
        floor.set_transform(translation(0.0, -1.0, 0.0)).unwrap();
        floor.material.reflective = 0.5;
        floor.material.transparency = 0.5;
        floor.material.refractive_index = 1.5;
        w.objects.push(floor.clone());
        let mut ball = sphere().unwrap();
        ball.material.color = color(1.0, 0.0, 0.0);
        ball.material.ambient = 0.5;
        ball.set_transform(translation(0.0, -3.5, -0.5)).unwrap();
        w.objects.push(ball);
        let r = ray(
            &point(0.0, 0.0, -3.0),
            &vector(0.0, -Float::sqrt(2.0) / 2.0, Float::sqrt(2.0) / 2.0),
        );
        let xs = intersections(vec![Intersection::new(Float::sqrt(2.0), floor)]);
        let comps = prepare_computations(&xs[0], &r, &xs);
        let c = w.shade_hit(&comps, 5);
        assert_eq!(c, color(0.93391, 0.69643, 0.69243));
    }
}

#[cfg(test)]
mod test_multiple_light_sources {
    use super::*;

    #[test]
    fn world_can_have_multiple_light_sources() {
        let mut w = world();
        let l1 = point_light(&point(0.0, 0.0, 0.0), &color(1.0, 1.0, 1.0));
        let l2 = point_light(&point(1.0, 1.0, 1.0), &color(1.0, 1.0, 1.0));
        w.add_light(&l1);
        w.add_light(&l2);
        assert_eq!(w.lights, vec![l1, l2]);
    }

    #[test]
    fn shade_hit_adds_color_from_all_lights() {
        let mut w = default_world();
        w.add_light(&point_light(&point(10.0, 10.0, 10.), &color(0.0, 1.0, 0.0)));
        let r = ray(&point(0.0, 0.0, -5.0), &vector(0.0, 0.0, 1.0));
        let shape = w.objects.first().unwrap();
        let i = intersection(4.0, shape.clone());
        let comps = i.prepare_computations(&r, &intersections(vec![i.clone()]));
        let c = w.shade_hit(&comps, 0);

        assert_eq!(c, color(0.38065, 0.57582, 0.28549));
    }
}

#[cfg(test)]
mod test_soft_shadows {
    use super::*;

    #[test]
    fn is_shadow_tests_for_occlusion_between_two_points() {
        let w = default_world();
        let light_position = point(-10., -10., -10.);

        let examples = [
            (point(-10., -10., 10.), false),
            (point(10., 10., 10.), true),
            (point(-20., -20., -20.), false),
            (point(-5., -5., -5.), false),
        ];

        for (point, result) in examples {
            assert_eq!(w.is_shadowed(&light_position, &point), result);
        }
    }

    #[test]
    fn point_lights_evaluate_the_light_intensity_at_a_given_point() {
        let w = default_world();
        let light = w.lights.first().unwrap();

        let examples = [
            (point(0.0, 1.0001, 0.0), 1.0),
            (point(-1.0001, 0.0, 0.0), 1.0),
            (point(0.0, 0.0, -1.0001), 1.0),
            (point(0.0, 0.0, 1.0001), 0.0),
            (point(1.0001, 0.0, 0.0), 0.0),
            (point(0.0, -1.0001, 0.0), 0.0),
            (point(0.0, 0.0, 0.0), 0.0),
        ];

        for (pt, result) in examples {
            let intensity = light.intensity_at(&pt, &w);
            assert_eq!(intensity, result);
        }
    }

    #[test]
    fn lighting_uses_light_intensity_to_attenuate_color() {
        let mut w = default_world();
        let light = point_light(&point(0.0, 0.0, -10.0), &color(1.0, 1.0, 1.0));
        w.set_lights(vec![light]);
        let mut shape = w.objects.first().unwrap().clone();
        shape.material.ambient = 0.1;
        shape.material.diffuse = 0.9;
        shape.material.specular = 0.0;
        shape.material.color = color(1.0, 1.0, 1.0);
        w.objects[0] = shape.clone();
        let pt = point(0.0, 0.0, -1.0);
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);

        let examples = [
            (1.0, color(1.0, 1.0, 1.0)),
            (0.5, color(0.55, 0.55, 0.55)),
            (0.0, color(0.1, 0.1, 0.1)),
        ];

        for (intensity, result) in examples {
            let lighting = shape.lighting(&light, &pt, &eyev, &normalv, intensity);
            assert_eq!(lighting, result);
        }
    }
}
