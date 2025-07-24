use crate::{
    cone::*, cube::*, cylinder::*, group::*, intersection::*, object::*, plane::*, ray::*,
    sphere::*, tuple::*,
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
    pub fn local_normal_at(&self, p: &Point) -> Point {
        match self {
            Shape::Sphere(sphere) => sphere.local_normal_at(p),
            Shape::Plane(plane) => plane.local_normal_at(p),
            Shape::Cube(cube) => cube.local_normal_at(p),
            Shape::Cylinder(cylinder) => cylinder.local_normal_at(p),
            Shape::Cone(cone) => cone.local_normal_at(p),
            Shape::Group(group) => group.local_normal_at(p),
        }
    }

    pub fn local_intersect(&self, ray: &Ray, object: &Object) -> Result<Intersections, String> {
        match self {
            Shape::Sphere(sphere) => Ok(sphere.local_intersect(ray, object)),
            Shape::Plane(plane) => Ok(plane.local_intersect(ray, object)),
            Shape::Cube(cube) => Ok(cube.local_intersect(ray, object)),
            Shape::Cylinder(cylinder) => Ok(cylinder.local_intersect(ray, object)),
            Shape::Cone(cone) => Ok(cone.local_intersect(ray, object)),
            Shape::Group(group) => group.local_intersect(ray, object),
        }
    }

    pub fn add_child(&mut self, child: &mut Object) {
        match self {
            Shape::Group(group) => group.add_child(child),
            _ => unreachable!(),
        }
    }
}
