use std::{cmp::Ordering, ops::Index};

use crate::{float::*, object::*, ray::*, tuple::*};

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Intersection {
    pub t: Float,
    pub object: Object,
}

impl Intersection {
    pub fn new(t: Float, object: Object) -> Self {
        Intersection { t, object }
    }

    pub fn positive(&self) -> bool {
        self.t > 0.
    }

    pub fn prepare_computations(&self, ray: &Ray) -> Result<PreparedComputations, String> {
        let point = ray.position(self.t);
        let eyev = -ray.direction;
        let mut normalv = self.object.normal_at(&point)?;
        let inside = normalv.dot(&eyev) < 0.0;
        if inside {
            normalv = -normalv;
        }
        let over_point = point + normalv * EPSILON;
        Ok(PreparedComputations {
            t: self.t,
            object: self.object,
            point,
            over_point,
            eyev,
            normalv,
            inside,
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

    pub fn inner(&self) -> &Vec<Intersection> {
        &self.0
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn hit(&self) -> Option<Intersection> {
        self.0.iter().find(|i| i.positive()).cloned()
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
    pub eyev: Vector,
    pub normalv: Vector,
    pub inside: bool,
}

pub fn intersection(t: Float, object: &Object) -> Intersection {
    Intersection::new(t, object.to_owned())
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
) -> Result<PreparedComputations, String> {
    intersection.prepare_computations(ray)
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
        let r = ray(point(0., 0., -5.), vector(0., 0., 1.));
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
        let xs = intersections(vec![i2, i1]);
        let i = hit(&xs);
        assert_eq!(i, Some(i1));
        assert_eq!(xs.hit(), Some(i1));
    }

    #[test]
    fn the_hit_when_some_intersections_have_negative_t() {
        let s = sphere();
        let i1 = intersection(-1., &s);
        let i2 = intersection(1., &s);
        let xs = intersections(vec![i2, i1]);
        let i = hit(&xs);
        assert_eq!(i, Some(i2));
        assert_eq!(xs.hit(), Some(i2));
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
        let xs = intersections(vec![i1, i2, i3, i4]);
        let i = hit(&xs);
        assert_eq!(i, Some(i4));
        assert_eq!(xs.hit(), Some(i4));
    }
}

#[cfg(test)]
mod test_chapter_7_world_intersections {
    #![allow(non_snake_case)]

    use super::*;

    use crate::sphere::*;

    #[test]
    fn precomputing_the_state_of_an_intersection() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let shape = sphere();
        let i = intersection(4.0, &shape);
        let comps = prepare_computations(&i, &r).unwrap();
        assert_eq!(comps.t, i.t);
        assert_eq!(comps.object, i.object);
        assert_eq!(comps.point, point(0.0, 0.0, -1.0));
        assert_eq!(comps.eyev, vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, vector(0.0, 0.0, -1.0));

        let comps2 = i.prepare_computations(&r).unwrap();
        assert_eq!(comps, comps2);
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_outside() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let shape = sphere();
        let i = intersection(4.0, &shape);
        let comps = prepare_computations(&i, &r).unwrap();
        assert_eq!(comps.inside, false);
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_inside() {
        let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let shape = sphere();
        let i = intersection(1.0, &shape);
        let comps = prepare_computations(&i, &r).unwrap();
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
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let shape = Object::new(translation(0.0, 0.0, 1.0));
        let i = intersection(5.0, &shape);
        let comps = prepare_computations(&i, &r).unwrap();
        assert!(comps.over_point.z < -EPSILON / 2.0);
        assert!(comps.point.z > comps.over_point.z);
    }
}
