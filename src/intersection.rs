use std::{cmp::Ordering, mem, ops::Index};

use crate::{float::*, object::*, ray::*, tuple::*};

#[derive(PartialEq, Clone, Debug)]
pub struct Intersection {
    pub t: Float,
    pub object: Object,
}

impl Intersection {
    pub fn new(t: Float, object: &Object) -> Self {
        Intersection {
            t,
            object: object.to_owned(),
        }
    }

    pub fn positive(&self) -> bool {
        self.t > 0.
    }

    pub fn prepare_computations(
        &self,
        ray: &Ray,
        xs: &Intersections,
    ) -> Result<PreparedComputations, String> {
        let point = ray.position(self.t);
        let eyev = -ray.direction;
        let mut normalv = self.object.normal_at(&point)?;
        let inside = normalv.dot(&eyev) < 0.0;
        if inside {
            normalv = -normalv;
        }
        let over_point = point + normalv * EPSILON;
        let under_point = point - normalv * EPSILON;
        let reflectv = ray.direction.reflect(&normalv);
        let cos_i = eyev.dot(&normalv);

        let mut n1: Float = 1.0;
        let mut n2: Float = 1.0;

        let mut containers: Vec<Object> = vec![];
        for i in xs.inner().iter() {
            if i == self {
                n1 = containers
                    .last()
                    .map(|object| object.material.refractive_index)
                    .unwrap_or(1.0);
            }

            if containers.contains(&i.object) {
                containers.retain(|element| *element != i.object)
            } else {
                containers.push(i.object.clone());
            }

            if i == self {
                n2 = containers
                    .last()
                    .map(|object| object.material.refractive_index)
                    .unwrap_or(1.0);
                break;
            }
        }
        let n_ratio = n1 / n2;
        let sin2_t = n_ratio.powf(2.0) * (1.0 - cos_i.powf(2.0));

        Ok(PreparedComputations {
            t: self.t,
            object: self.object.clone(),
            point,
            over_point,
            under_point,
            eyev,
            normalv,
            reflectv,
            cos_i,
            sin2_t,
            inside,
            n1,
            n2,
            n_ratio,
        })
    }
}

impl PartialOrd for Intersection {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.t.partial_cmp(&other.t)
    }
}

#[derive(PartialEq, PartialOrd, Clone, Debug)]
pub struct Intersections(Vec<Intersection>);

impl Intersections {
    pub fn empty() -> Self {
        Intersections(vec![])
    }

    pub fn new(mut intersections: Vec<Intersection>) -> Self {
        intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        Intersections(intersections)
    }

    pub fn from_object(intersections: Vec<Float>, object: &Object) -> Self {
        Self::new(
            intersections
                .iter()
                .map(|t| Intersection::new(*t, object))
                .collect(),
        )
    }

    pub fn inner(&self) -> &Vec<Intersection> {
        &self.0
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn hit(&self) -> Option<Intersection> {
        self.0.iter().find(|i| i.positive()).cloned()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Index<usize> for Intersections {
    type Output = Intersection;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct PreparedComputations {
    pub t: Float,
    pub object: Object,
    pub point: Point,
    pub over_point: Point,
    pub under_point: Point,
    pub eyev: Vector,
    pub normalv: Vector,
    pub reflectv: Vector,
    pub cos_i: Float,
    pub sin2_t: Float,
    pub inside: bool,
    pub n1: Float,
    pub n2: Float,
    pub n_ratio: Float,
}

impl PreparedComputations {
    pub fn schlick(&self) -> Float {
        // Find the cosine of the angle between the eye and normal vectors.
        let mut cos = self.cos_i;

        // Total internal reflection can only occur if n1 > n2
        if self.n1 > self.n2 {
            if self.sin2_t > 1.0 {
                return 1.0;
            }

            // Compute cosine of theta_t using trig identity.
            cos = f64::sqrt(1.0 - self.sin2_t);
        }

        let r0 = ((self.n1 - self.n2) / (self.n1 + self.n2)).powf(2.0);

        r0 + (1.0 - r0) * (1.0 - cos).powf(5.0)
    }
}

pub trait CylinderIntersection {
    fn minimum(&self) -> Float;
    fn maximum(&self) -> Float;
    fn radius(&self, plane: Float) -> Float;

    fn intersect_walls(&self, a: Float, b: Float, c: Float, ray: &Ray) -> Vec<Float> {
        let discriminant = b * b - 4.0 * a * c;

        // Ray does not intersect the cylinder
        if discriminant < 0.0 {
            return vec![];
        }

        // Ray intersects with the cylinder
        let t0 = &mut ((-b - f64::sqrt(discriminant)) / (2.0 * a));
        let t1 = &mut ((-b + f64::sqrt(discriminant)) / (2.0 * a));
        if t0 > t1 {
            mem::swap(t0, t1);
        }

        let mut xs = vec![];

        let y0 = ray.origin.y + *t0 * ray.direction.y;
        if self.minimum() < y0 && y0 < self.maximum() {
            xs.push(*t0);
        }
        let y1 = ray.origin.y + *t1 * ray.direction.y;
        if self.minimum() < y1 && y1 < self.maximum() {
            xs.push(*t1);
        }

        xs
    }

    fn intersect_caps(&self, ray: &Ray) -> Vec<Float> {
        let mut xs = vec![];

        // Caps only matter if the cylinder might possibly be
        // intersected by the ray.
        if ray.direction.y == 0.0 {
            return vec![];
        }

        // Check for an intersection with the lower end cap by intersecting
        // the ray with the plane at y=cyl.minimum
        if let Some(t) = self.check_cap(ray, self.minimum()) {
            xs.push(t);
        }

        // Check for an intersection with the upper end cap by intersecting
        // the ray with the plane at y=cyl.maximum
        if let Some(t) = self.check_cap(ray, self.maximum()) {
            xs.push(t);
        }

        xs
    }

    // A helper function to reduce duplication.
    // checks to see if the intersection at `t` is within a radius
    // of 1 (the radius of your cylinders) from the y axis.
    fn check_cap(&self, ray: &Ray, plane: Float) -> Option<Float> {
        let t = (plane - ray.origin.y) / ray.direction.y;
        let x = ray.origin.x + t * ray.direction.x;
        let z = ray.origin.z + t * ray.direction.z;
        if (x.powf(2.0) + z.powf(2.0)) <= self.radius(plane) {
            Some(t)
        } else {
            None
        }
    }
}

pub fn intersection(t: Float, object: &Object) -> Intersection {
    Intersection::new(t, object)
}

pub fn intersections(intersections: Vec<Intersection>) -> Intersections {
    Intersections::new(intersections)
}

pub fn hit(intersections: &Intersections) -> Option<Intersection> {
    intersections.hit()
}

pub fn prepare_computations(
    intersection: &Intersection,
    ray: &Ray,
    xs: &Intersections,
) -> Result<PreparedComputations, String> {
    intersection.prepare_computations(ray, xs)
}

#[cfg(test)]
mod test_chapter_5_intersections {
    #![allow(non_snake_case)]

    use super::*;

    use crate::sphere::*;

    #[test]
    fn an_intersection_encapsulates_t_and_object() {
        let s = sphere();
        let i = intersection(3.5, &s);
        assert_eq!(i.t, 3.5);
        assert_eq!(i.object, s);
    }

    #[test]
    fn aggregating_intersections() {
        let s = sphere();
        let i1 = intersection(1., &s);
        let i2 = intersection(2., &s);
        let xs = intersections(vec![i1, i2]);
        assert_eq!(xs.len(), 2);
        assert_eq_float!(xs[0].t, 1.0);
        assert_eq_float!(xs[1].t, 2.0);
    }

    #[test]
    fn intersect_sets_the_object_on_the_intersection() {
        let r = ray(&point(0., 0., -5.), &vector(0., 0., 1.));
        let s = sphere();
        let xs = s.intersect(&r).unwrap();
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].object, s);
        assert_eq!(xs[1].object, s);
    }

    #[test]
    fn the_hit_when_all_intersections_have_positive_t() {
        let s = sphere();
        let i1 = intersection(1., &s);
        let i2 = intersection(2., &s);
        let xs = intersections(vec![i2, i1.clone()]);
        let i = hit(&xs);
        assert_eq!(i.unwrap(), i1);
        assert_eq!(xs.hit().unwrap(), i1);
    }

    #[test]
    fn the_hit_when_some_intersections_have_negative_t() {
        let s = sphere();
        let i1 = intersection(-1., &s);
        let i2 = intersection(1., &s);
        let xs = intersections(vec![i2.clone(), i1]);
        let i = hit(&xs);
        assert_eq!(i.unwrap(), i2);
        assert_eq!(xs.hit().unwrap(), i2);
    }

    #[test]
    fn the_hit_when_all_intersections_have_negative_t() {
        let s = sphere();
        let i1 = intersection(-2., &s);
        let i2 = intersection(-1., &s);
        let xs = intersections(vec![i2, i1]);
        let i = hit(&xs);
        assert_eq!(i, None);
        assert_eq!(xs.hit(), None);
    }

    #[test]
    fn the_hit_is_always_the_lowest_nonnegative_intersection() {
        let s = sphere();
        let i1 = intersection(5., &s);
        let i2 = intersection(7., &s);
        let i3 = intersection(-3., &s);
        let i4 = intersection(2., &s);
        let xs = intersections(vec![i1, i2, i3, i4.clone()]);
        let i = hit(&xs);
        assert_eq!(i.unwrap(), i4);
        assert_eq!(xs.hit().unwrap(), i4);
    }
}

#[cfg(test)]
mod test_chapter_7_world_intersections {
    #![allow(non_snake_case)]

    use super::*;

    use crate::sphere::*;

    #[test]
    fn precomputing_the_state_of_an_intersection() {
        let r = ray(&point(0.0, 0.0, -5.0), &vector(0.0, 0.0, 1.0));
        let shape = sphere();
        let i = intersection(4.0, &shape);
        let comps = prepare_computations(&i, &r, &intersections(vec![i.clone()])).unwrap();
        assert_eq!(comps.t, i.t);
        assert_eq!(comps.object, i.object);
        assert_eq!(comps.point, point(0.0, 0.0, -1.0));
        assert_eq!(comps.eyev, vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, vector(0.0, 0.0, -1.0));

        let comps2 = i
            .prepare_computations(&r, &intersections(vec![i.clone()]))
            .unwrap();
        assert_eq!(comps, comps2);
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_outside() {
        let r = ray(&point(0.0, 0.0, -5.0), &vector(0.0, 0.0, 1.0));
        let shape = sphere();
        let i = intersection(4.0, &shape);
        let comps = prepare_computations(&i, &r, &intersections(vec![i.clone()])).unwrap();
        assert_eq!(comps.inside, false);
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_inside() {
        let r = ray(&point(0.0, 0.0, 0.0), &vector(0.0, 0.0, 1.0));
        let shape = sphere();
        let i = intersection(1.0, &shape);
        let comps = prepare_computations(&i, &r, &intersections(vec![i.clone()])).unwrap();
        assert_eq!(comps.point, point(0.0, 0.0, 1.0));
        assert_eq!(comps.eyev, vector(0.0, 0.0, -1.0));
        assert_eq!(comps.inside, true);
        // normal would have been (0, 0, 1), but is inverted!
        assert_eq!(comps.normalv, vector(0.0, 0.0, -1.0));
    }
}

#[cfg(test)]
mod test_chapter_8_shadows {
    #![allow(non_snake_case)]

    use super::*;

    use crate::matrix::*;

    #[test]
    fn the_hit_should_offset_the_point() {
        let r = ray(&point(0.0, 0.0, -5.0), &vector(0.0, 0.0, 1.0));
        let shape = Object::new(translation(0.0, 0.0, 1.0));
        let i = intersection(5.0, &shape);
        let comps = prepare_computations(&i, &r, &intersections(vec![i.clone()])).unwrap();
        assert!(comps.over_point.z < -EPSILON / 2.0);
        assert!(comps.point.z > comps.over_point.z);
    }
}

#[cfg(test)]
mod test_chapter_11_reflection {
    #![allow(non_snake_case)]

    use super::*;

    use crate::{matrix::*, plane::*, sphere::*};

    #[test]
    fn precomputing_the_reflection_vector() {
        let shape = plane();
        let r = ray(
            &point(0.0, 1.0, -1.0),
            &vector(0.0, -2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0),
        );
        let i = intersection(2_f64.sqrt(), &shape);
        let comps = prepare_computations(&i, &r, &intersections(vec![i.clone()])).unwrap();
        assert_eq!(
            comps.reflectv,
            vector(0.0, 2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0)
        );
    }

    #[test]
    fn finding_n1_and_n2_at_various_intersections() {
        let mut A = glass_sphere();
        A.set_transform(scaling(2.0, 2.0, 2.0));
        A.material.refractive_index = 1.5;
        let mut B = glass_sphere();
        B.set_transform(translation(0.0, 0.0, -0.25));
        B.material.refractive_index = 2.0;
        let mut C = glass_sphere();
        C.set_transform(translation(0.0, 0.0, 0.25));
        C.material.refractive_index = 2.5;
        let r = ray(&point(0.0, 0.0, -4.0), &vector(0.0, 0.0, 1.0));
        let xs = intersections(vec![
            Intersection::new(2.0, &A),
            Intersection::new(2.75, &B),
            Intersection::new(3.25, &C),
            Intersection::new(4.75, &B),
            Intersection::new(5.25, &C),
            Intersection::new(6.0, &A),
        ]);

        let examples = vec![
            (1.0, 1.5),
            (1.5, 2.0),
            (2.0, 2.5),
            (2.5, 2.5),
            (2.5, 1.5),
            (1.5, 1.0),
        ];
        for (idx, (n1, n2)) in examples.iter().enumerate() {
            let comps = prepare_computations(&xs[idx], &r, &xs).unwrap();
            assert_eq_float!(comps.n1, n1);
            assert_eq_float!(comps.n2, n2);
        }
    }

    #[test]
    fn the_under_point_is_offset_below_the_surface() {
        let r = ray(&point(0.0, 0.0, -5.0), &vector(0.0, 0.0, 1.0));
        let mut shape = glass_sphere();
        shape.set_transform(translation(0.0, 0.0, 1.0));
        let i = intersection(5.0, &shape);
        let xs = intersections(vec![i.clone()]);
        let comps = i.prepare_computations(&r, &xs).unwrap();
        assert!(comps.under_point.z > EPSILON / 2.0);
        assert!(comps.point.z < comps.under_point.z);
    }

    #[test]
    fn the_schlick_approximation_under_total_internal_reflection() {
        let shape = glass_sphere();
        let r = ray(
            &point(0.0, 0.0, f64::sqrt(2.0) / 2.0),
            &vector(0.0, 1.0, 0.0),
        );
        let xs = intersections(vec![
            Intersection::new(-f64::sqrt(2.0) / 2.0, &shape),
            Intersection::new(f64::sqrt(2.0) / 2.0, &shape),
        ]);
        let comps = prepare_computations(&xs[1], &r, &xs).unwrap();
        let reflectance = comps.schlick();
        assert_eq_float!(reflectance, 1.0);
    }

    #[test]
    fn the_schlick_approximation_with_a_perpendicular_viewing_angle() {
        let shape = glass_sphere();
        let r = ray(&point(0.0, 0.0, 0.0), &vector(0.0, 1.0, 0.0));
        let xs = intersections(vec![
            Intersection::new(-1.0, &shape),
            Intersection::new(1.0, &shape),
        ]);
        let comps = prepare_computations(&xs[1], &r, &xs).unwrap();
        let reflectance = comps.schlick();
        assert_eq_float!(reflectance, 0.04);
    }

    #[test]
    fn the_schlick_approximation_with_small_angle_and_n2_greater_than_n1() {
        let shape = glass_sphere();
        let r = ray(&point(0.0, 0.99, -2.0), &vector(0.0, 0.0, 1.0));
        let xs = intersections(vec![Intersection::new(1.8589, &shape)]);
        let comps = prepare_computations(&xs[0], &r, &xs).unwrap();
        let reflectance = comps.schlick();
        assert_eq_float!(reflectance, 0.48873);
    }
}
