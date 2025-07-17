use std::{cmp::Ordering, ops::Index};

use crate::{float::*, sphere::*};

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Intersection {
    pub t: Float,
    pub object: Sphere,
}

impl Intersection {
    pub fn new(t: Float, object: Sphere) -> Self {
        Intersection { t, object }
    }

    pub fn positive(&self) -> bool {
        self.t > 0.
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

pub fn intersection(t: Float, object: Sphere) -> Intersection {
    Intersection::new(t, object)
}

pub fn intersections(intersections: Vec<Intersection>) -> Intersections {
    Intersections::new(intersections)
}

pub fn hit(intersections: &Intersections) -> Option<Intersection> {
    intersections.hit()
}

#[cfg(test)]
mod test_chapter_5_intersections {
    #![allow(non_snake_case)]

    use super::*;
    use crate::{ray::*, tuple::*};

    #[test]
    fn an_intersection_encapsulates_t_and_object() {
        let s = sphere();
        let i = intersection(3.5, s.clone());
        assert_eq!(i.t, 3.5);
        assert_eq!(i.object, s);
    }

    #[test]
    fn aggregating_intersections() {
        let s = sphere();
        let i1 = intersection(1., s.clone());
        let i2 = intersection(2., s);
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
        let i1 = intersection(1., s.clone());
        let i2 = intersection(2., s);
        let xs = intersections(vec![i2, i1]);
        let i = hit(&xs);
        assert_eq!(i, Some(i1));
        assert_eq!(xs.hit(), Some(i1));
    }

    #[test]
    fn the_hit_when_some_intersections_have_negative_t() {
        let s = sphere();
        let i1 = intersection(-1., s.clone());
        let i2 = intersection(1., s);
        let xs = intersections(vec![i2, i1]);
        let i = hit(&xs);
        assert_eq!(i, Some(i2));
        assert_eq!(xs.hit(), Some(i2));
    }

    #[test]
    fn the_hit_when_all_intersections_have_negative_t() {
        let s = sphere();
        let i1 = intersection(-2., s.clone());
        let i2 = intersection(-1., s);
        let xs = intersections(vec![i2, i1]);
        let i = hit(&xs);
        assert_eq!(i, None);
        assert_eq!(xs.hit(), None);
    }

    #[test]
    fn the_hit_is_always_the_lowest_nonnegative_intersection() {
        let s = sphere();
        let i1 = intersection(5., s.clone());
        let i2 = intersection(7., s.clone());
        let i3 = intersection(-3., s.clone());
        let i4 = intersection(2., s);
        let xs = intersections(vec![i1, i2, i3, i4]);
        let i = hit(&xs);
        assert_eq!(i, Some(i4));
        assert_eq!(xs.hit(), Some(i4));
    }
}
