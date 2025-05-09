use crate::{float::*, ray::*, tuple::*};

#[derive(Clone)]
pub struct Sphere {}

impl Sphere {
    pub fn new() -> Self {
        Sphere {}
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Float> {
        let sphere_to_ray = ray.origin - point(0., 0., 0.);
        let a = ray.direction.dot(&ray.direction);
        let b = 2. * ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.;
        let discriminant = b * b - 4. * a * c;

        if discriminant < 0. {
            return vec![];
        }

        let t1 = (-b - discriminant.sqrt()) / (2. * a);
        let t2 = (-b + discriminant.sqrt()) / (2. * a);

        vec![t1, t2]
    }
}

pub fn sphere() -> Sphere {
    Sphere::new()
}

pub fn intersect(sphere: &Sphere, ray: &Ray) -> Vec<Float> {
    sphere.intersect(ray)
}
