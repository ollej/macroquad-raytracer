use crate::{color::*, light::*, sphere::*, tuple::*};

pub fn world() -> World {
    World::new()
}

#[derive(PartialEq, Clone, Debug)]
pub struct World {
    objects: Vec<Sphere>,
    light: Option<Light>,
}

impl World {
    pub fn new() -> Self {
        Self {
            objects: vec![],
            light: None,
        }
    }
}

#[cfg(test)]
mod test_chapter_7_world {
    use super::*;

    #[test]
    fn creating_a_world() {
        let w = world();
        assert_eq!(w.objects, vec![]);
        assert_eq!(w.light, None);
    }
}
