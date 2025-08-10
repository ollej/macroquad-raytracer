use crate::{color::*, float::*, tuple::*, world::*};

pub fn point_light(origin: &Point, intensity: &Color) -> Light {
    Light::new(origin.to_owned(), intensity.to_owned())
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Light {
    pub position: Point,
    pub intensity: Color,
}

impl Light {
    pub fn new(position: Point, intensity: Color) -> Self {
        Light {
            position,
            intensity,
        }
    }

    pub fn intensity_at(&self, point: &Point, world: &World) -> Float {
        if world.is_shadowed(&self.position, point) {
            0.0
        } else {
            1.0
        }
    }
}

#[cfg(test)]
mod test_chapter_6_light {
    use super::*;

    #[test]
    fn a_point_light_has_a_position_and_intensity() {
        let intensity = Color::new(1., 1., 1.);
        let position = point(0., 0., 0.);
        let light = point_light(&position, &intensity);
        assert_eq!(light.position, position);
        assert_eq!(light.intensity, intensity);
    }
}
