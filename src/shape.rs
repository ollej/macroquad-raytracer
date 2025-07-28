use crate::{
    bounds::*, cone::*, cube::*, cylinder::*, group::*, intersection::*, object::*, plane::*,
    ray::*, sphere::*, triangle::*, tuple::*,
};

#[derive(PartialEq, Clone, Debug)]
pub enum Shape {
    Sphere(Sphere),
    Plane(Plane),
    Cube(Cube),
    Cylinder(Cylinder),
    Cone(Cone),
    Group(Group),
    Triangle(Triangle),
    SmoothTriangle(SmoothTriangle),
}

impl Shape {
    pub fn local_normal_at(&self, p: &Point, hit: Option<Intersection>) -> Point {
        match self {
            Shape::Sphere(sphere) => sphere.local_normal_at(p, hit),
            Shape::Plane(plane) => plane.local_normal_at(p, hit),
            Shape::Cube(cube) => cube.local_normal_at(p, hit),
            Shape::Cylinder(cylinder) => cylinder.local_normal_at(p, hit),
            Shape::Cone(cone) => cone.local_normal_at(p, hit),
            Shape::Group(group) => group.local_normal_at(p, hit),
            Shape::Triangle(triangle) => triangle.local_normal_at(p, hit),
            Shape::SmoothTriangle(smooth_triangle) => smooth_triangle.local_normal_at(p, hit),
        }
    }

    pub fn local_intersect(&self, ray: &Ray, object: &Object) -> Intersections {
        match self {
            Shape::Sphere(sphere) => sphere.local_intersect(ray, object),
            Shape::Plane(plane) => plane.local_intersect(ray, object),
            Shape::Cube(cube) => cube.local_intersect(ray, object),
            Shape::Cylinder(cylinder) => cylinder.local_intersect(ray, object),
            Shape::Cone(cone) => cone.local_intersect(ray, object),
            Shape::Group(group) => group.local_intersect(ray, object),
            Shape::Triangle(triangle) => triangle.local_intersect(ray, object),
            Shape::SmoothTriangle(smooth_triangle) => smooth_triangle.local_intersect(ray, object),
        }
    }

    pub fn add_child(&mut self, child: &mut Object) {
        match self {
            Shape::Group(group) => group.add_child(child),
            _ => unreachable!(),
        }
    }

    pub fn update_parents(&mut self, parent: &mut Object) {
        match self {
            Shape::Group(group) => group.update_parents(parent),
            _ => (),
        }
    }

    pub fn bounding_box(&self) -> BoundingBox {
        match self {
            Shape::Sphere(sphere) => sphere.bounding_box(),
            Shape::Plane(plane) => plane.bounding_box(),
            Shape::Cube(cube) => cube.bounding_box(),
            Shape::Cylinder(cylinder) => cylinder.bounding_box(),
            Shape::Cone(cone) => cone.bounding_box(),
            Shape::Group(group) => group.bounding_box(),
            Shape::Triangle(triangle) => triangle.bounding_box(),
            Shape::SmoothTriangle(smooth_triangle) => smooth_triangle.bounding_box(),
        }
    }
}
