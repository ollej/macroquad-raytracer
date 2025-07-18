use crate::{float::*, ray::*, sphere::*, tuple::*};

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Shape {
    Sphere(Sphere),
}

impl Shape {
    pub fn normal_at(&self, p: &Point) -> Point {
        match self {
            Shape::Sphere(sphere) => sphere.normal_at(p),
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<(Float, Float)> {
        match self {
            Shape::Sphere(sphere) => sphere.intersect(ray),
        }
    }
}
