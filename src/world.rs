use crate::{
    color::*, intersection::*, light::*, material::*, matrix::*, object::*, ray::*, sphere::*,
    tuple::*,
};

pub fn world() -> World {
    World::new()
}

pub fn default_world() -> World {
    World::default()
}

pub fn intersect_world(world: &World, ray: &Ray) -> Result<Intersections, String> {
    world.intersect(ray)
}

pub fn shade_hit(
    world: &World,
    prepared_computations: &PreparedComputations,
    remaining: usize,
) -> Result<Color, String> {
    world.shade_hit(prepared_computations, remaining)
}

pub fn color_at(world: &World, ray: &Ray, remaining: usize) -> Result<Color, String> {
    world.color_at(ray, remaining)
}

pub fn is_shadowed(world: &World, point: &Point) -> bool {
    world.is_shadowed(point)
}

#[derive(PartialEq, Clone, Debug)]
pub struct World {
    pub objects: Vec<Object>,
    pub light: Option<Light>,
}

impl Default for World {
    fn default() -> World {
        let mut s1 = sphere();
        let m = Material {
            color: color(0.8, 1.0, 0.6),
            diffuse: 0.7,
            specular: 0.2,
            ..Default::default()
        };
        s1.set_material(&m);
        let mut s2 = sphere();
        s2.set_transform(&Matrix::scaling(0.5, 0.5, 0.5));
        World {
            objects: vec![s1, s2],
            light: Some(point_light(
                &point(-10.0, 10.0, -10.),
                &color(1.0, 1.0, 1.0),
            )),
        }
    }
}

impl World {
    pub fn new() -> Self {
        Self {
            objects: vec![],
            light: None,
        }
    }

    pub fn set_light(&mut self, light: &Light) {
        self.light = Some(light.to_owned());
    }

    pub fn contains(&self, object: &Object) -> bool {
        self.objects.contains(object)
    }

    pub fn intersect(&self, ray: &Ray) -> Result<Intersections, String> {
        let mut all_intersections = vec![];
        for obj in self.objects.iter() {
            let mut intersections = obj.intersect(ray)?.inner().to_owned();
            all_intersections.append(&mut intersections);
        }
        Ok(Intersections::new(all_intersections))
    }

    pub fn shade_hit(
        &self,
        comps: &PreparedComputations,
        remaining: usize,
    ) -> Result<Color, String> {
        let shadowed = self.is_shadowed(&comps.over_point);

        let surface_color = match &self.light {
            Some(light) => comps.object.material.lighting(
                &comps.object,
                light,
                &comps.over_point,
                &comps.eyev,
                &comps.normalv,
                shadowed,
            )?,
            None => BLACK,
        };

        let reflected_color = self.reflected_color(&comps, remaining)?;
        let refracted_color = self.refracted_color(&comps, remaining)?;

        let material = comps.object.material;
        if material.reflective > 0.0 && material.transparency > 0.0 {
            let reflectance = comps.schlick();
            return Ok(surface_color
                + reflected_color * reflectance
                + refracted_color * (1.0 - reflectance));
        }

        Ok(surface_color + reflected_color + refracted_color)
    }

    pub fn color_at(&self, ray: &Ray, remaining: usize) -> Result<Color, String> {
        let intersections = self.intersect(ray)?;
        match intersections.hit() {
            Some(hit) => {
                let prepared_computations = hit.prepare_computations(ray, &intersections)?;
                self.shade_hit(&prepared_computations, remaining)
            }
            None => Ok(BLACK),
        }
    }

    pub fn is_shadowed(&self, point: &Point) -> bool {
        if let Some(light) = &self.light {
            let v = light.position - point;
            let distance = v.magnitude();
            let direction = v.normalize();
            let r = ray(point, &direction);
            self.intersect(&r)
                .ok()
                .map(|intersections| intersections.hit())
                .flatten()
                .map(|hit| hit.t < distance)
                .unwrap_or(false)
        } else {
            false
        }
    }

    fn reflected_color(
        &self,
        comps: &PreparedComputations,
        remaining: usize,
    ) -> Result<Color, String> {
        if remaining < 1 {
            Ok(BLACK)
        } else if comps.object.material.reflective == 0.0 {
            Ok(BLACK)
        } else {
            let reflect_ray = ray(&comps.over_point, &comps.reflectv);
            self.color_at(&reflect_ray, remaining - 1)
                .map(|c| c * comps.object.material.reflective)
        }
    }

    fn refracted_color(
        &self,
        comps: &PreparedComputations,
        remaining: usize,
    ) -> Result<Color, String> {
        if remaining == 0 || comps.object.material.transparency == 0.0 {
            return Ok(BLACK);
        }

        let n_ratio = comps.n_ratio();
        let cos_i = comps.cos_i();
        let sin2_t = n_ratio * n_ratio * (1.0 - cos_i * cos_i);

        if sin2_t > 1.0 {
            return Ok(BLACK);
        }

        // Find cos(theta_t) via trigonometric identity
        let cos_t = f64::sqrt(1.0 - sin2_t);
        // Compute the direction of the refracted ray
        let direction = comps.normalv * (n_ratio * cos_i - cos_t) - comps.eyev * n_ratio;
        // Create the refracted ray
        let refract_ray = Ray::new(comps.under_point, direction);
        // Find the color of the refracted ray, making sure to multiply
        // by the transparency value to account for any opacity
        let color =
            self.color_at(&refract_ray, remaining - 1)? * comps.object.material.transparency;
        Ok(color)
    }
}

#[cfg(test)]
mod test_chapter_7_world {
    use super::*;

    #[test]
    fn creating_a_world() {
        let w = world();
        assert_eq!(w.objects, vec![]);
        assert_eq!(w.light, None);
    }

    #[test]
    fn the_default_world() {
        let light = point_light(&point(-10.0, 10.0, -10.), &color(1.0, 1.0, 1.0));
        let mut s1 = sphere();
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
        let mut s2 = sphere();
        s2.set_transform(&Matrix::scaling(0.5, 0.5, 0.5));
        let w = default_world();
        assert_eq!(w.light, Some(light));
        assert!(w.contains(&s1));
        assert!(w.contains(&s2));

        let w2 = World::default();
        assert_eq!(w, w2);
    }

    #[test]
    fn intersect_a_world_with_a_ray() {
        let w = default_world();
        let r = ray(&point(0.0, 0.0, -5.0), &vector(0.0, 0.0, 1.0));
        let xs = intersect_world(&w, &r).unwrap();
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
        let i = intersection(4.0, &shape);
        let comps = i.prepare_computations(&r, &intersections(vec![i])).unwrap();
        let c = shade_hit(&w, &comps, 0).unwrap();
        assert_eq!(c, color(0.38066, 0.47583, 0.2855));

        let c2 = w.shade_hit(&comps, 0).unwrap();
        assert_eq!(c, c2);
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let mut w = default_world();
        w.set_light(&point_light(&point(0.0, 0.25, 0.0), &color(1.0, 1.0, 1.0)));
        let r = ray(&point(0.0, 0.0, 0.0), &vector(0.0, 0.0, 1.0));
        let shape = w.objects.get(1).unwrap();
        let i = intersection(0.5, &shape);
        let comps = i.prepare_computations(&r, &intersections(vec![i])).unwrap();
        let c = w.shade_hit(&comps, 0).unwrap();
        assert_eq!(c, color(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn the_color_when_a_ray_misses() {
        let w = default_world();
        let r = ray(&point(0.0, 0.0, -5.0), &vector(0.0, 1.0, 0.0));
        let c = color_at(&w, &r, 0).unwrap();
        assert_eq!(c, color(0.0, 0.0, 0.0));

        let c2 = w.color_at(&r, 0).unwrap();
        assert_eq!(c, c2);
    }

    #[test]
    fn the_color_when_a_ray_hits() {
        let w = default_world();
        let r = ray(&point(0.0, 0.0, -5.0), &vector(0.0, 0.0, 1.0));
        let c = w.color_at(&r, 0).unwrap();
        assert_eq!(c, color(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn the_color_with_an_intersection_behind_the_ray() {
        let mut w = default_world();
        let mut outer = w.objects[0];
        outer.material.ambient = 1.0;
        let mut inner = w.objects[1];
        inner.material.ambient = 1.0;
        w.objects[1] = inner;
        let r = ray(&point(0.0, 0.0, 0.75), &vector(0.0, 0.0, -1.0));
        let c = w.color_at(&r, 0).unwrap();
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
        assert_eq!(is_shadowed(&w, &p), false);
    }

    #[test]
    fn the_shadow_when_an_object_is_between_the_point_and_the_light() {
        let w = default_world();
        let p = point(10.0, -10.0, 10.0);
        assert_eq!(is_shadowed(&w, &p), true);
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_light() {
        let w = default_world();
        let p = point(-20.0, 20.0, -20.0);
        assert_eq!(is_shadowed(&w, &p), false);
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_point() {
        let w = default_world();
        let p = point(-2.0, 2.0, -2.0);
        assert_eq!(is_shadowed(&w, &p), false);
    }

    #[test]
    fn shade_hit_is_given_an_intersection_in_shadow() {
        let mut w = world();
        w.set_light(&point_light(&point(0.0, 0.0, -10.0), &color(1.0, 1.0, 1.0)));
        let s1 = sphere();
        w.objects.push(s1);
        let s2 = Object::new_sphere(translation(0.0, 0.0, 10.0), Material::default());
        w.objects.push(s2);
        let r = ray(&point(0.0, 0.0, 5.0), &vector(0.0, 0.0, 1.0));
        let i = intersection(4.0, &s2);
        let comps = prepare_computations(&i, &r, &intersections(vec![i])).unwrap();
        let c = w.shade_hit(&comps, 0).unwrap();
        assert_eq!(c, color(0.1, 0.1, 0.1));
    }
}

#[cfg(test)]
mod test_chapter_11_reflection {
    #![allow(non_snake_case)]

    use super::*;

    use crate::plane::*;

    use crate::pattern::test_pattern;

    #[test]
    fn the_reflected_color_for_a_nonreflective_material() {
        let w = default_world();
        let r = ray(&point(0.0, 0.0, 0.0), &vector(0.0, 0.0, 1.0));
        let mut shape = w.objects[1];
        shape.material.ambient = 1.0;
        let i = intersection(1.0, &shape);
        let comps = i.prepare_computations(&r, &intersections(vec![i])).unwrap();
        let c = w.reflected_color(&comps, 0).unwrap();
        assert_eq!(c, color(0.0, 0.0, 0.0));
    }

    #[test]
    fn the_reflected_color_for_a_reflective_material() {
        let mut w = default_world();
        let mut shape = plane();
        shape.material.reflective = 0.5;
        shape.set_transform(&translation(0.0, -1.0, 0.0));
        w.objects.push(shape);
        let r = ray(
            &point(0.0, 0.0, -3.0),
            &vector(0.0, -2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0),
        );
        let i = intersection(2.0_f64.sqrt(), &shape);
        let comps = i.prepare_computations(&r, &intersections(vec![i])).unwrap();
        let c = w.reflected_color(&comps, 1).unwrap();
        assert_eq!(c, color(0.1903322, 0.237915, 0.142749));
    }

    #[test]
    fn the_shade_hit_for_a_reflective_material() {
        let mut w = default_world();
        let mut shape = plane();
        shape.material.reflective = 0.5;
        shape.set_transform(&translation(0.0, -1.0, 0.0));
        w.objects.push(shape);
        let r = ray(
            &point(0.0, 0.0, -3.0),
            &vector(0.0, -2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0),
        );
        let i = intersection(2.0_f64.sqrt(), &shape);
        let comps = i.prepare_computations(&r, &intersections(vec![i])).unwrap();
        let c = w.shade_hit(&comps, 1).unwrap();
        assert_eq!(c, color(0.876757, 0.924340, 0.829174));
    }

    #[test]
    fn color_at_with_mutually_reflective_surfaces() {
        let mut w = world();
        w.set_light(&point_light(&point(0.0, 0.0, 0.0), &color(1.0, 1.0, 1.0)));
        let mut lower = plane();
        lower.material.reflective = 1.0;
        lower.set_transform(&translation(0.0, -1.0, 0.0));
        w.objects.push(lower);
        let mut upper = plane();
        upper.material.reflective = 1.0;
        upper.set_transform(&translation(0.0, 1.0, 0.0));
        w.objects.push(upper);
        let r = ray(&point(0.0, 0.0, 0.0), &vector(0.0, 1.0, 0.0));
        let c = w.color_at(&r, 1).unwrap();
        assert_eq!(c, color(3.8, 3.8, 3.8));
    }

    #[test]
    fn the_reflected_color_at_the_maximum_recursive_depth() {
        let mut w = default_world();
        let mut shape = plane();
        shape.material.reflective = 0.5;
        shape.set_transform(&translation(0.0, -1.0, 0.0));
        w.objects.push(shape);
        let r = ray(
            &point(0.0, 0.0, -3.0),
            &vector(0.0, -2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0),
        );
        let i = intersection(2.0_f64.sqrt(), &shape);
        let comps = i.prepare_computations(&r, &intersections(vec![i])).unwrap();
        let c = w.reflected_color(&comps, 0).unwrap();
        assert_eq!(c, color(0.0, 0.0, 0.0));
    }

    #[test]
    fn the_refracted_color_with_an_opaque_surface() {
        let w = default_world();
        let shape = w.objects.first().unwrap();
        let r = ray(&point(0.0, 0.0, -5.0), &vector(0.0, 0.0, 1.0));
        let xs = intersections(vec![
            Intersection::new(4.0, *shape),
            Intersection::new(6.0, *shape),
        ]);
        let comps = prepare_computations(&xs[0], &r, &xs).unwrap();
        let c = w.refracted_color(&comps, 5).unwrap();
        assert_eq!(c, color(0.0, 0.0, 0.0));
    }

    #[test]
    fn the_refracted_color_at_the_maximum_recursive_depth() {
        let w = default_world();
        let mut shape = w.objects[0];
        shape.material.transparency = 1.0;
        shape.material.refractive_index = 1.5;
        let r = ray(&point(0.0, 0.0, -5.0), &vector(0.0, 0.0, 1.0));
        let xs = intersections(vec![
            Intersection::new(4.0, shape),
            Intersection::new(6.0, shape),
        ]);
        let comps = prepare_computations(&xs[0], &r, &xs).unwrap();
        let c = w.refracted_color(&comps, 0).unwrap();
        assert_eq!(c, color(0.0, 0.0, 0.0));
    }

    #[test]
    fn the_refracted_color_under_total_internal_reflection() {
        let w = default_world();
        let mut shape = w.objects[0];
        shape.material.transparency = 1.0;
        shape.material.refractive_index = 1.5;
        let r = ray(
            &point(0.0, 0.0, 2.0_f64.sqrt() / 2.0),
            &vector(0.0, 1.0, 0.0),
        );
        let xs = intersections(vec![
            Intersection::new(-2.0_f64.sqrt() / 2.0, shape),
            Intersection::new(2.0_f64.sqrt() / 2.0, shape),
        ]);
        // NOTE: this time you're inside the sphere, so you need;
        // to look at the second intersection, xs[1], not xs[0];
        let comps = prepare_computations(&xs[1], &r, &xs).unwrap();
        let c = w.refracted_color(&comps, 5).unwrap();
        assert_eq!(c, color(0.0, 0.0, 0.0));
    }

    #[test]
    fn the_refracted_color_with_a_refracted_ray() {
        let mut w = default_world();
        let mut A = w.objects[0];
        A.material.ambient = 1.0;
        A.material.set_pattern(test_pattern());
        w.objects[0] = A;
        let mut B = w.objects[1];
        B.material.transparency = 1.0;
        B.material.refractive_index = 1.5;
        w.objects[1] = B;
        let r = ray(&point(0.0, 0.0, 0.1), &vector(0.0, 1.0, 0.0));
        let xs = intersections(vec![
            Intersection::new(-0.9899, A),
            Intersection::new(-0.4899, B),
            Intersection::new(0.4899, B),
            Intersection::new(0.9899, A),
        ]);
        let comps = prepare_computations(&xs[2], &r, &xs).unwrap();
        let c = w.refracted_color(&comps, 5).unwrap();
        assert_eq!(c, color(0.0, 0.99888, 0.04725));
    }

    #[test]
    fn shade_hit_with_a_transparent_material() {
        let mut w = default_world();
        let mut floor = plane();
        floor.set_transform(&translation(0.0, -1.0, 0.0));
        floor.material.transparency = 0.5;
        floor.material.refractive_index = 1.5;
        w.objects.push(floor);
        let mut ball = sphere();
        ball.material.color = color(1.0, 0.0, 0.0);
        ball.material.ambient = 0.5;
        ball.set_transform(&translation(0.0, -3.5, -0.5));
        w.objects.push(ball);
        let r = ray(
            &point(0.0, 0.0, -3.0),
            &vector(0.0, -f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0),
        );
        let xs = intersections(vec![Intersection::new(f64::sqrt(2.0), floor)]);
        let comps = prepare_computations(&xs[0], &r, &xs).unwrap();
        let c = w.shade_hit(&comps, 5).unwrap();
        assert_eq!(c, color(0.93642, 0.68642, 0.68642));
    }

    #[test]
    fn shade_hit_with_a_reflective_transparent_material() {
        let mut w = default_world();

        let mut floor = plane();
        floor.set_transform(&translation(0.0, -1.0, 0.0));
        floor.material.reflective = 0.5;
        floor.material.transparency = 0.5;
        floor.material.refractive_index = 1.5;
        w.objects.push(floor);
        let mut ball = sphere();
        ball.material.color = color(1.0, 0.0, 0.0);
        ball.material.ambient = 0.5;
        ball.set_transform(&translation(0.0, -3.5, -0.5));
        w.objects.push(ball);
        let r = ray(
            &point(0.0, 0.0, -3.0),
            &vector(0.0, -f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0),
        );
        let xs = intersections(vec![Intersection::new(f64::sqrt(2.0), floor)]);
        let comps = prepare_computations(&xs[0], &r, &xs).unwrap();
        let c = w.shade_hit(&comps, 5).unwrap();
        assert_eq!(c, color(0.93391, 0.69643, 0.69243));
    }
}
