use crate::{
    bounds::*, intersection::*, material::*, matrix::IDENTITY_MATRIX, object::*, ray::*, tuple::*,
};

pub trait TriangleIntersection {
    fn normal(&self) -> Vector;
    fn p1(&self) -> Vector;
    fn e1(&self) -> Vector;
    fn e2(&self) -> Vector;

    fn local_intersect(&self, ray: &Ray, object: &Object) -> Intersections {
        let dir_cross_e2 = ray.direction.cross(&self.e2());
        let det = self.e1().dot(&dir_cross_e2);
        if det.abs() == 0.0 {
            return Intersections::empty();
        }

        let f = 1.0 / det;
        let p1_to_origin = ray.origin - self.p1();
        let u = f * p1_to_origin.dot(&dir_cross_e2);
        if u < 0.0 || u > 1.0 {
            return Intersections::empty();
        }

        let origin_cross_e1 = p1_to_origin.cross(&self.e1());
        let v = f * ray.direction.dot(&origin_cross_e1);
        if v < 0.0 || (u + v) > 1.0 {
            return Intersections::empty();
        }

        let t = f * self.e2().dot(&origin_cross_e1);
        Intersections::new(vec![Intersection::with_uv(t, object.to_owned(), u, v)])
    }

    fn local_normal_at(&self, _p: &Point, _hit: Option<Intersection>) -> Vector {
        self.normal()
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Triangle {
    pub p1: Point,
    pub p2: Point,
    pub p3: Point,
    pub e1: Vector,
    pub e2: Vector,
    pub normal: Vector,
}

impl Triangle {
    pub fn new(p1: Point, p2: Point, p3: Point) -> Self {
        let e1 = p2 - p1;
        let e2 = p3 - p1;
        let normal = e2.cross(&e1).normalize();
        Self {
            p1,
            p2,
            p3,
            e1,
            e2,
            normal,
        }
    }
}

impl TriangleIntersection for Triangle {
    fn normal(&self) -> Vector {
        self.normal
    }
    fn p1(&self) -> Vector {
        self.p1
    }
    fn e1(&self) -> Vector {
        self.e1
    }
    fn e2(&self) -> Vector {
        self.e2
    }
}

impl Bounds for Triangle {
    fn bounding_box(&self) -> BoundingBox {
        BoundingBox::empty() + self.p1 + self.p2 + self.p3
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct SmoothTriangle {
    pub p1: Point,
    pub p2: Point,
    pub p3: Point,
    pub n1: Vector,
    pub n2: Vector,
    pub n3: Vector,
    pub e1: Vector,
    pub e2: Vector,
    pub normal: Vector,
}

impl SmoothTriangle {
    pub fn new(p1: Point, p2: Point, p3: Point, n1: Vector, n2: Vector, n3: Vector) -> Self {
        let e1 = p2 - p1;
        let e2 = p3 - p1;
        let normal = e2.cross(&e1).normalize();
        Self {
            p1,
            p2,
            p3,
            n1,
            n2,
            n3,
            e1,
            e2,
            normal,
        }
    }
}

impl TriangleIntersection for SmoothTriangle {
    fn normal(&self) -> Vector {
        self.normal
    }

    fn p1(&self) -> Vector {
        self.p1
    }

    fn e1(&self) -> Vector {
        self.e1
    }

    fn e2(&self) -> Vector {
        self.e2
    }

    fn local_normal_at(&self, _p: &Point, hit: Option<Intersection>) -> Vector {
        if let Some(hit) = hit {
            self.n2 * hit.u + self.n3 * hit.v + self.n1 * (1.0 - hit.u - hit.v)
        } else {
            self.normal()
        }
    }
}

impl Bounds for SmoothTriangle {
    fn bounding_box(&self) -> BoundingBox {
        BoundingBox::empty() + self.p1 + self.p2 + self.p3
    }
}

pub fn triangle(p1: Point, p2: Point, p3: Point) -> Result<Object, String> {
    Object::new_triangle(p1, p2, p3, IDENTITY_MATRIX, Material::default())
}

pub fn smooth_triangle(
    p1: Point,
    p2: Point,
    p3: Point,
    n1: Vector,
    n2: Vector,
    n3: Vector,
) -> Result<Object, String> {
    Object::new_smooth_triangle(p1, p2, p3, n1, n2, n3, IDENTITY_MATRIX, Material::default())
}

#[cfg(test)]
mod test_chapter_15_triangles {
    use super::*;

    use crate::{float::*, shape::*, test_common::*};

    fn test_triangle() -> Triangle {
        Triangle::new(
            point(0.0, 1.0, 0.0),
            point(-1.0, 0.0, 0.0),
            point(1.0, 0.0, 0.0),
        )
    }

    fn test_smooth_triangle() -> Object {
        let p1 = point(0.0, 1.0, 0.0);
        let p2 = point(-1.0, 0.0, 0.0);
        let p3 = point(1.0, 0.0, 0.0);
        let n1 = vector(0.0, 1.0, 0.0);
        let n2 = vector(-1.0, 0.0, 0.0);
        let n3 = vector(1.0, 0.0, 0.0);
        smooth_triangle(p1, p2, p3, n1, n2, n3).unwrap()
    }

    #[test]
    fn constructing_a_triangle() {
        let p1 = point(0.0, 1.0, 0.0);
        let p2 = point(-1.0, 0.0, 0.0);
        let p3 = point(1.0, 0.0, 0.0);
        let t = Triangle::new(p1, p2, p3);
        assert_eq!(t.p1, p1);
        assert_eq!(t.p2, p2);
        assert_eq!(t.p3, p3);
        assert_eq!(t.e1, vector(-1.0, -1.0, 0.0));
        assert_eq!(t.e2, vector(1.0, -1.0, 0.0));
        assert_eq!(t.normal, vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn finding_the_normal_on_a_triangle() {
        let t = test_triangle();
        let n1 = t.local_normal_at(&point(0.0, 0.5, 0.0), None);
        let n2 = t.local_normal_at(&point(-0.5, 0.75, 0.0), None);
        let n3 = t.local_normal_at(&point(0.5, 0.25, 0.0), None);
        assert_eq!(n1, t.normal);
        assert_eq!(n2, t.normal);
        assert_eq!(n3, t.normal);
    }

    #[test]
    fn intersecting_a_ray_parallel_to_the_triangle() {
        let t = test_triangle();
        let r = ray(&point(0.0, -1.0, -2.0), &vector(0.0, 1.0, 0.0));
        let xs = t.local_intersect(&r, &test_shape());
        assert!(xs.is_empty());
    }

    #[test]
    fn a_ray_misses_the_p1_p3_edge() {
        let t = test_triangle();
        let r = ray(&point(1.0, 1.0, -2.0), &vector(0.0, 0.0, 1.0));
        let xs = t.local_intersect(&r, &test_shape());
        assert!(xs.is_empty());
    }

    #[test]
    fn a_ray_misses_the_p1_p2_edge() {
        let t = test_triangle();
        let r = ray(&point(-1.0, 1.0, -2.0), &vector(0.0, 0.0, 1.0));
        let xs = t.local_intersect(&r, &test_shape());
        assert!(xs.is_empty());
    }

    #[test]
    fn a_ray_misses_the_p2_p3_edge() {
        let t = test_triangle();
        let r = ray(&point(0.0, -1.0, -2.0), &vector(0.0, 0.0, 1.0));
        let xs = t.local_intersect(&r, &test_shape());
        assert!(xs.is_empty());
    }

    #[test]
    fn a_ray_strikes_a_triangle() {
        let t = test_triangle();
        let r = ray(&point(0.0, 0.5, -2.0), &vector(0.0, 0.0, 1.0));
        let xs = t.local_intersect(&r, &test_shape());
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 2.0);
    }

    #[test]
    fn a_triangle_has_a_bounding_box() {
        let p1 = point(-3.0, 7.0, 2.0);
        let p2 = point(6.0, 2.0, -4.0);
        let p3 = point(2.0, -1.0, -1.0);
        let shape = Triangle::new(p1, p2, p3);
        let b = shape.bounding_box();
        assert_eq!(b.minimum, point(-3.0, -1.0, -4.0));
        assert_eq!(b.maximum, point(6.0, 7.0, 2.0));
    }

    #[test]
    fn create_a_triangle_object() {
        let p1 = point(0.0, 1.0, 0.0);
        let p2 = point(-1.0, 0.0, 0.0);
        let p3 = point(1.0, 0.0, 0.0);
        let t = Object::new_triangle(p1, p2, p3, IDENTITY_MATRIX, Material::default()).unwrap();
        match t.shape {
            Shape::Triangle(triangle) => {
                assert_eq!(triangle.p1, p1);
                assert_eq!(triangle.p2, p2);
                assert_eq!(triangle.p3, p3);
            }
            _ => panic!("Object is not a Triangle"),
        }
    }

    #[test]
    fn constructing_a_smooth_triangle() {
        let p1 = point(0.0, 1.0, 0.0);
        let p2 = point(-1.0, 0.0, 0.0);
        let p3 = point(1.0, 0.0, 0.0);
        let n1 = vector(0.0, 1.0, 0.0);
        let n2 = vector(-1.0, 0.0, 0.0);
        let n3 = vector(1.0, 0.0, 0.0);
        let tri = SmoothTriangle::new(p1, p2, p3, n1, n2, n3);
        assert_eq!(tri.p1, p1);
        assert_eq!(tri.p2, p2);
        assert_eq!(tri.p3, p3);
        assert_eq!(tri.n1, n1);
        assert_eq!(tri.n2, n2);
        assert_eq!(tri.n3, n3);
    }

    #[test]
    fn an_intersection_with_a_smooth_triangle_stores_u_and_v() {
        let tri = test_smooth_triangle();
        let r = ray(&point(-0.2, 0.3, -2.0), &vector(0.0, 0.0, 1.0));
        let xs = tri.intersect(&r);
        assert_eq_float!(xs[0].u, 0.45);
        assert_eq_float!(xs[0].v, 0.25);
    }

    #[test]
    fn a_smooth_triangle_uses_u_v_to_interpolate_the_normal() {
        let tri = test_smooth_triangle();
        let i = intersection_with_uv(1.0, tri.clone(), 0.45, 0.25);
        let n = tri.normal_at(&point(0.0, 0.0, 0.0), Some(i));
        assert_eq!(n, vector(-0.5547, 0.83205, 0.0));
    }

    #[test]
    fn preparing_the_normal_on_a_smooth_triangle() {
        let tri = test_smooth_triangle();
        let i = intersection_with_uv(1.0, tri, 0.45, 0.25);
        let r = ray(&point(-0.2, 0.3, -2.0), &vector(0.0, 0.0, 1.0));
        let xs = intersections(vec![i.clone()]);
        let comps = i.prepare_computations(&r, &xs);
        assert_eq!(comps.normalv, vector(-0.5547, 0.83205, 0.0));
    }
}
