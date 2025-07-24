use crate::{
    cone::*, cube::*, cylinder::*, float::*, group::*, plane::*, ray::*, sphere::*, tuple::*,
};

#[derive(PartialEq, Clone, Debug)]
pub enum Shape {
    Sphere(Sphere),
    Plane(Plane),
    Cube(Cube),
    Cylinder(Cylinder),
    Cone(Cone),
    Group(Group),
}

impl Shape {
    pub fn normal_at(&self, p: &Point) -> Point {
        match self {
            Shape::Sphere(sphere) => sphere.local_normal_at(p),
            Shape::Plane(plane) => plane.local_normal_at(p),
            Shape::Cube(cube) => cube.local_normal_at(p),
            Shape::Cylinder(cylinder) => cylinder.local_normal_at(p),
            Shape::Cone(cone) => cone.local_normal_at(p),
            Shape::Group(group) => group.local_normal_at(p),
        }
    }

    pub fn local_intersect(&self, ray: &Ray) -> Vec<Float> {
        match self {
            Shape::Sphere(sphere) => sphere.local_intersect(ray),
            Shape::Plane(plane) => plane.local_intersect(ray),
            Shape::Cube(cube) => cube.local_intersect(ray),
            Shape::Cylinder(cylinder) => cylinder.local_intersect(ray),
            Shape::Cone(cone) => cone.local_intersect(ray),
            Shape::Group(group) => group.local_intersect(ray),
        }
    }
}
