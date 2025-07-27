use crate::{
    bounds::*, intersection::*, material::*, matrix::IDENTITY_MATRIX, object::*, ray::*, tuple::*,
};

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

    pub fn local_intersect(&self, ray: &Ray, object: &Object) -> Intersections {
        let dir_cross_e2 = ray.direction.cross(&self.e2);
        let det = self.e1.dot(&dir_cross_e2);
        if det.abs() == 0.0 {
            return Intersections::empty();
        }

        let f = 1.0 / det;
        let p1_to_origin = ray.origin - self.p1;
        let u = f * p1_to_origin.dot(&dir_cross_e2);
        if u < 0.0 || u > 1.0 {
            return Intersections::empty();
        }

        let origin_cross_e1 = p1_to_origin.cross(&self.e1);
        let v = f * ray.direction.dot(&origin_cross_e1);
        if v < 0.0 || (u + v) > 1.0 {
            return Intersections::empty();
        }

        let t = f * self.e2.dot(&origin_cross_e1);
        Intersections::new(vec![Intersection::new(t, object)])
    }

    pub fn local_normal_at(&self, _p: &Point) -> Vector {
        self.normal
    }
}

impl Bounds for Triangle {
    fn bounding_box(&self) -> BoundingBox {
        BoundingBox::empty() + self.p1 + self.p2 + self.p3
    }
}

pub fn triangle(p1: &Point, p2: &Point, p3: &Point) -> Object {
    Object::new_triangle(
        p1.to_owned(),
        p2.to_owned(),
        p3.to_owned(),
        IDENTITY_MATRIX,
        Material::default(),
    )
}

#[cfg(test)]
mod test_chapter_15_triangles {
    use super::*;

    use crate::shape::*;

    fn test_triangle() -> Triangle {
        Triangle::new(
            point(0.0, 1.0, 0.0),
            point(-1.0, 0.0, 0.0),
            point(1.0, 0.0, 0.0),
        )
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
        let n1 = t.local_normal_at(&point(0.0, 0.5, 0.0));
        let n2 = t.local_normal_at(&point(-0.5, 0.75, 0.0));
        let n3 = t.local_normal_at(&point(0.5, 0.25, 0.0));
        assert_eq!(n1, t.normal);
        assert_eq!(n2, t.normal);
        assert_eq!(n3, t.normal);
    }

    #[test]
    fn intersecting_a_ray_parallel_to_the_triangle() {
        let t = test_triangle();
        let r = ray(&point(0.0, -1.0, -2.0), &vector(0.0, 1.0, 0.0));
        let xs = t.local_intersect(&r, &Object::empty());
        assert!(xs.is_empty());
    }

    #[test]
    fn a_ray_misses_the_p1_p3_edge() {
        let t = test_triangle();
        let r = ray(&point(1.0, 1.0, -2.0), &vector(0.0, 0.0, 1.0));
        let xs = t.local_intersect(&r, &Object::empty());
        assert!(xs.is_empty());
    }

    #[test]
    fn a_ray_misses_the_p1_p2_edge() {
        let t = test_triangle();
        let r = ray(&point(-1.0, 1.0, -2.0), &vector(0.0, 0.0, 1.0));
        let xs = t.local_intersect(&r, &Object::empty());
        assert!(xs.is_empty());
    }

    #[test]
    fn a_ray_misses_the_p2_p3_edge() {
        let t = test_triangle();
        let r = ray(&point(0.0, -1.0, -2.0), &vector(0.0, 0.0, 1.0));
        let xs = t.local_intersect(&r, &Object::empty());
        assert!(xs.is_empty());
    }

    #[test]
    fn a_ray_strikes_a_triangle() {
        let t = test_triangle();
        let r = ray(&point(0.0, 0.5, -2.0), &vector(0.0, 0.0, 1.0));
        let xs = t.local_intersect(&r, &Object::empty());
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
        let t = Object::new_triangle(p1, p2, p3, IDENTITY_MATRIX, Material::default());
        match t.shape {
            Shape::Triangle(triangle) => {
                assert_eq!(triangle.p1, p1);
                assert_eq!(triangle.p2, p2);
                assert_eq!(triangle.p3, p3);
            }
            _ => panic!("Object is not a Triangle"),
        }
    }
}
