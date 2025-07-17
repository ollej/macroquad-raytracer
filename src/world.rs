use crate::{
    color::*, intersection::*, light::*, material::*, matrix::*, ray::*, sphere::*, tuple::*,
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

pub fn shade_hit(world: &World, prepared_computations: &PreparedComputations) -> Color {
    world.shade_hit(prepared_computations)
}

pub fn color_at(world: &World, ray: &Ray) -> Result<Color, String> {
    world.color_at(ray)
}

#[derive(PartialEq, Clone, Debug)]
pub struct World {
    pub objects: Vec<Sphere>,
    pub light: Option<Light>,
}

impl Default for World {
    fn default() -> World {
        let mut s1 = sphere();
        let m = Material::new(color(0.8, 1.0, 0.6), 0.1, 0.7, 0.2, 200.0);
        s1.set_material(&m);
        let mut s2 = sphere();
        s2.set_transform(&Matrix::scaling(0.5, 0.5, 0.5));
        World {
            objects: vec![s1, s2],
            light: Some(point_light(point(-10.0, 10.0, -10.), color(1.0, 1.0, 1.0))),
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

    pub fn contains(&self, object: &Sphere) -> bool {
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

    pub fn shade_hit(&self, prepared_computations: &PreparedComputations) -> Color {
        match &self.light {
            Some(light) => prepared_computations.object.material.lighting(
                light,
                &prepared_computations.point,
                &prepared_computations.eyev,
                &prepared_computations.normalv,
            ),
            None => BLACK,
        }
    }

    pub fn color_at(&self, ray: &Ray) -> Result<Color, String> {
        let intersections = self.intersect(ray)?;
        match intersections.hit() {
            Some(hit) => {
                let prepared_computations = hit.prepare_computations(ray)?;
                Ok(self.shade_hit(&prepared_computations))
            }
            None => Ok(BLACK),
        }
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
        let light = point_light(point(-10.0, 10.0, -10.), color(1.0, 1.0, 1.0));
        let mut s1 = sphere();
        let m = Material::new(color(0.8, 1.0, 0.6), 0.1, 0.7, 0.2, 200.0);
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
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
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
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let shape = w.objects.first().unwrap();
        let i = intersection(4.0, &shape);
        let comps = i.prepare_computations(&r).unwrap();
        let c = shade_hit(&w, &comps);
        assert_eq!(c, color(0.38066, 0.47583, 0.2855));

        let c2 = w.shade_hit(&comps);
        assert_eq!(c, c2);
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let mut w = default_world();
        w.set_light(&point_light(point(0.0, 0.25, 0.0), color(1.0, 1.0, 1.0)));
        let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let shape = w.objects.get(1).unwrap();
        let i = intersection(0.5, &shape);
        let comps = i.prepare_computations(&r).unwrap();
        let c = w.shade_hit(&comps);
        assert_eq!(c, color(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn the_color_when_a_ray_misses() {
        let w = default_world();
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 1.0, 0.0));
        let c = color_at(&w, &r).unwrap();
        assert_eq!(c, color(0.0, 0.0, 0.0));

        let c2 = w.color_at(&r).unwrap();
        assert_eq!(c, c2);
    }

    #[test]
    fn the_color_when_a_ray_hits() {
        let w = default_world();
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let c = w.color_at(&r).unwrap();
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
        let r = ray(point(0.0, 0.0, 0.75), vector(0.0, 0.0, -1.0));
        let c = w.color_at(&r).unwrap();
        assert_eq!(c, inner.material.color);
    }
}
