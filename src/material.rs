use crate::{color::*, float::*};

pub fn material() -> Material {
    Material::default()
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Material {
    pub color: Color,
    pub ambient: Float,
    pub diffuse: Float,
    pub specular: Float,
    pub shininess: Float,
}

impl Material {
    pub fn new(
        color: Color,
        ambient: Float,
        diffuse: Float,
        specular: Float,
        shininess: Float,
    ) -> Self {
        Material {
            color,
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }
}

impl Default for Material {
    fn default() -> Self {
        Material::new(color(1., 1., 1.), 0.1, 0.9, 0.9, 200.0)
    }
}

#[cfg(test)]
mod test_chapter_6_material {
    use super::*;

    #[test]
    fn the_default_material() {
        let m = material();
        assert_eq!(m.color, color(1., 1., 1.));
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
    }
}
