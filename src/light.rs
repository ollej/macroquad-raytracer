use crate::{color::*, float::*, tuple::*, world::*};

pub fn point_light(origin: &Point, intensity: &Color) -> Light {
    Light::point_light(origin, intensity)
}

pub fn area_light(
    corner: &Point,
    full_uvec: &Vector,
    usteps: usize,
    full_vvec: &Vector,
    vsteps: usize,
    intensity: &Color,
) -> Light {
    Light::area_light(corner, full_uvec, usteps, full_vvec, vsteps, intensity)
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum LightType {
    PointLight(PointLight),
    AreaLight(AreaLight),
}

impl LightType {
    pub fn intensity_at(&self, light: &Light, point: &Point, world: &World) -> Float {
        match self {
            LightType::PointLight(point_light) => point_light.intensity_at(light, point, world),
            LightType::AreaLight(area_light) => area_light.intensity_at(light, point, world),
        }
    }

    pub fn point_on_light(&self, light: &Light, u: usize, v: usize) -> Point {
        match self {
            LightType::PointLight(_point_light) => light.position,
            LightType::AreaLight(area_light) => area_light.point_on_light(u, v),
        }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Light {
    pub light_type: LightType,
    pub position: Point,
    pub intensity: Color,
}

impl Light {
    pub fn new(position: Point, intensity: Color, light_type: LightType) -> Self {
        Self {
            position,
            intensity,
            light_type,
        }
    }

    pub fn point_light(origin: &Point, intensity: &Color) -> Light {
        Self::new(
            origin.to_owned(),
            intensity.to_owned(),
            LightType::PointLight(PointLight {}),
        )
    }

    pub fn area_light(
        corner: &Point,
        full_uvec: &Vector,
        usteps: usize,
        full_vvec: &Vector,
        vsteps: usize,
        intensity: &Color,
    ) -> Light {
        Self::new(
            *corner + full_uvec / 2.0 + full_vvec / 2.0,
            intensity.to_owned(),
            LightType::AreaLight(AreaLight::new(
                corner, full_uvec, usteps, full_vvec, vsteps, intensity,
            )),
        )
    }

    pub fn point_on_light(&self, u: usize, v: usize) -> Point {
        self.light_type.point_on_light(self, u, v)
    }

    pub fn intensity_at(&self, point: &Point, world: &World) -> Float {
        self.light_type.intensity_at(self, point, world)
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct PointLight {}

impl PointLight {
    pub fn intensity_at(&self, light: &Light, point: &Point, world: &World) -> Float {
        if world.is_shadowed(&light.position, point) {
            0.0
        } else {
            1.0
        }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct AreaLight {
    corner: Point,
    uvec: Vector,
    usteps: usize,
    vvec: Vector,
    vsteps: usize,
    intensity: Color,
    samples: usize,
}

impl AreaLight {
    pub fn new(
        corner: &Point,
        full_uvec: &Vector,
        usteps: usize,
        full_vvec: &Vector,
        vsteps: usize,
        intensity: &Color,
    ) -> Self {
        Self {
            corner: corner.to_owned(),
            uvec: full_uvec / usteps as Float,
            usteps,
            vvec: full_vvec / vsteps as Float,
            vsteps,
            intensity: intensity.to_owned(),
            samples: usteps * vsteps,
        }
    }

    pub fn point_on_light(&self, u: usize, v: usize) -> Point {
        self.corner + self.uvec * (u as Float + 0.5) + self.vvec * (v as Float + 0.5)
    }

    pub fn intensity_at(&self, light: &Light, point: &Point, world: &World) -> Float {
        let mut total = 0.0;

        for v in 0..self.vsteps {
            for u in 0..self.usteps {
                let light_position = light.point_on_light(u, v);
                if !world.is_shadowed(&light_position, point) {
                    total += 1.0;
                }
            }
        }

        total / self.samples as Float
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

#[cfg(test)]
mod test_area_light {
    use super::*;

    #[test]
    fn creating_an_area_light() {
        let corner = point(0.0, 0.0, 0.0);
        let v1 = vector(2.0, 0.0, 0.0);
        let v2 = vector(0.0, 0.0, 1.0);
        let light = area_light(&corner, &v1, 4, &v2, 2, &color(1.0, 1.0, 1.0));
        match light.light_type {
            LightType::AreaLight(area_light) => {
                assert_eq!(area_light.corner, corner);
                assert_eq!(area_light.uvec, vector(0.5, 0.0, 0.0));
                assert_eq!(area_light.usteps, 4);
                assert_eq!(area_light.vvec, vector(0.0, 0.0, 0.5));
                assert_eq!(area_light.vsteps, 2);
                assert_eq!(area_light.samples, 8);
                assert_eq!(light.position, point(1.0, 0.0, 0.5));
            }
            _ => panic!("Light is not an area light"),
        }
    }

    #[test]
    fn finding_a_single_point_on_an_area_light() {
        let corner = point(0.0, 0.0, 0.0);
        let v1 = vector(2.0, 0.0, 0.0);
        let v2 = vector(0.0, 0.0, 1.0);
        let light = area_light(&corner, &v1, 4, &v2, 2, &color(1.0, 1.0, 1.0));

        let examples = [
            (0, 0, point(0.25, 0.0, 0.25)),
            (1, 0, point(0.75, 0.0, 0.25)),
            (0, 1, point(0.25, 0.0, 0.75)),
            (2, 0, point(1.25, 0.0, 0.25)),
            (3, 1, point(1.75, 0.0, 0.75)),
        ];

        for (u, v, result) in examples {
            let pt = light.point_on_light(u, v);
            assert_eq!(pt, result);
        }
    }

    #[test]
    fn the_area_light_intensity_function() {
        let w = default_world();
        let corner = point(-0.5, -0.5, -5.0);
        let v1 = vector(1.0, 0.0, 0.0);
        let v2 = vector(0.0, 1.0, 0.0);
        let light = area_light(&corner, &v1, 2, &v2, 2, &color(1.0, 1.0, 1.0));

        let examples = [
            (point(0.0, 0.0, 2.0), 0.0),
            (point(1.0, -1.0, 2.0), 0.25),
            (point(1.5, 0.0, 2.0), 0.5),
            (point(1.25, 1.25, 3.0), 0.75),
            (point(0.0, 0.0, -2.0), 1.0),
        ];

        for (pt, result) in examples {
            let intensity = light.intensity_at(&pt, &w);
            assert_eq!(intensity, result);
        }
    }
}
