use crate::{cube::*, cylinder::*, float::*, plane::*, ray::*, sphere::*, tuple::*};

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Shape {
    Sphere(Sphere),
    Plane(Plane),
    Cube(Cube),
    Cylinder(Cylinder),
}

impl Shape {
    pub fn normal_at(&self, p: &Point) -> Point {
        match self {
            Shape::Sphere(sphere) => sphere.normal_at(p),
            Shape::Plane(plane) => plane.normal_at(p),
            Shape::Cube(cube) => cube.normal_at(p),
            Shape::Cylinder(cylinder) => cylinder.normal_at(p),
        }
    }

    pub fn local_intersect(&self, ray: &Ray) -> Vec<Float> {
        match self {
            Shape::Sphere(sphere) => sphere.local_intersect(ray),
            Shape::Plane(plane) => plane.local_intersect(ray),
            Shape::Cube(cube) => cube.local_intersect(ray),
            Shape::Cylinder(cylinder) => cylinder.local_intersect(ray),
        }
    }
}
