use crate::{color::*, float::*, light::*, object::*, pattern::*, tuple::*};

pub const MAX_REFLECTIVE_DEPTH: usize = 4;

pub fn material() -> Material {
    Material::default()
}

pub fn lighting(
    material: &Material,
    object: &Object,
    light: &Light,
    point: &Point,
    eyev: &Vector,
    normalv: &Vector,
    light_intensity: Float,
) -> Color {
    material.lighting(object, light, point, eyev, normalv, light_intensity)
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Material {
    pub color: Color,
    pub ambient: Float,
    pub diffuse: Float,
    pub specular: Float,
    pub shininess: Float,
    pub reflective: Float,
    pub transparency: Float,
    pub refractive_index: Float,
    pub pattern: Option<Pattern>,
}

impl Material {
    pub fn new(
        color: Color,
        ambient: Float,
        diffuse: Float,
        specular: Float,
        shininess: Float,
        reflective: Float,
        transparency: Float,
        refractive_index: Float,
        pattern: Option<Pattern>,
    ) -> Self {
        Material {
            color,
            ambient,
            diffuse,
            specular,
            shininess,
            reflective,
            transparency,
            refractive_index,
            pattern,
        }
    }

    pub fn set_pattern(&mut self, pattern: Pattern) {
        self.pattern = Some(pattern);
    }

    pub fn lighting(
        &self,
        object: &Object,
        light: &Light,
        point: &Point,
        eyev: &Vector,
        normalv: &Vector,
        light_intensity: Float,
    ) -> Color {
        // Use color from pattern if available
        let color = if let Some(pattern) = self.pattern {
            pattern.pattern_at_object(object, point)
        } else {
            self.color
        };

        // Combine the surface color with the light's color/intensity
        let effective_color = color * light.intensity;

        // Compute the ambient contribution
        let ambient = effective_color * self.ambient;

        // Only return ambient light if point is in shadow
        if light_intensity == 0.0 {
            return ambient;
        }

        // Find the direction to the light source
        let lightv = (light.position - point).normalize();

        // light_dot_normal represents the cosine of the angle between the
        // light vector and the normal vector. A negative number means the
        // light is on the other side of the surface.
        let light_dot_normal = lightv.dot(normalv);
        let (diffuse, specular) = if light_dot_normal < 0. {
            (BLACK, BLACK)
        } else {
            // Compute the diffuse contribution
            let diffuse = effective_color * self.diffuse * light_dot_normal;

            // reflect_dot_eye represents the cosine of the angle between the
            // reflection vector and the eye vector. A negative number means the
            // light reflects away from the eye.
            let reflectv = (-lightv).reflect(normalv);
            let reflect_dot_eye = reflectv.dot(eyev);

            let specular: Color = if reflect_dot_eye <= 0. {
                BLACK
            } else {
                // Compute the specular contribution
                let factor = reflect_dot_eye.powf(self.shininess);
                light.intensity * self.specular * factor
            };

            (diffuse, specular)
        };

        // Add the three contributions together to get the final shading
        ambient + (diffuse * light_intensity) + (specular * light_intensity)
    }
}

impl Default for Material {
    fn default() -> Self {
        Material::new(
            color(1.0, 1.0, 1.0),
            0.1,
            0.9,
            0.9,
            200.0,
            0.0,
            0.0,
            1.0,
            None,
        )
    }
}

#[cfg(test)]
mod test_common {
    use crate::material::*;

    pub fn setup() -> (Material, Point) {
        (material(), point(0., 0., 0.))
    }
}

#[cfg(test)]
mod test_chapter_6_material {
    use super::*;

    use crate::sphere::*;

    #[test]
    fn the_default_material() {
        let (m, _position) = test_common::setup();
        assert_eq!(m.color, color(1., 1., 1.));
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
    }

    #[test]
    fn lighting_with_the_eye_between_the_light_and_the_surface() {
        let (m, position) = test_common::setup();
        let eyev = vector(0., 0., -1.);
        let normalv = vector(0., 0., -1.);
        let light = point_light(&point(0., 0., -10.), &color(1., 1., 1.));
        let sphere = sphere().unwrap();
        let result = lighting(&m, &sphere, &light, &position, &eyev, &normalv, 1.0);
        assert_eq!(result, color(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighting_with_the_eye_between_the_light_and_surface_eye_offset_45_degrees() {
        let (m, position) = test_common::setup();
        let eyev = vector(0., Float::sqrt(2.0) / 2.0, -Float::sqrt(2.0) / 2.0);
        let normalv = vector(0., 0., -1.);
        let light = point_light(&point(0., 0., -10.), &color(1., 1., 1.));
        let sphere = sphere().unwrap();
        let result = lighting(&m, &sphere, &light, &position, &eyev, &normalv, 1.0);
        assert_eq!(result, color(1.0, 1.0, 1.0));
    }

    #[test]
    fn lighting_with_the_eye_opposite_surface_light_offset_45_degrees() {
        let (m, position) = test_common::setup();
        let eyev = vector(0., 0., -1.);
        let normalv = vector(0., 0., -1.);
        let light = point_light(&point(0., 10., -10.), &color(1., 1., 1.));
        let sphere = sphere().unwrap();
        let result = lighting(&m, &sphere, &light, &position, &eyev, &normalv, 1.0);
        assert_eq!(result, color(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn lighting_with_eye_in_the_path_of_the_reflection_vector() {
        let (m, position) = test_common::setup();
        let eyev = vector(0., -Float::sqrt(2.0) / 2.0, -Float::sqrt(2.0) / 2.0);
        let normalv = vector(0., 0., -1.);
        let light = point_light(&point(0., 10., -10.), &color(1., 1., 1.));
        let sphere = sphere().unwrap();
        let result = lighting(&m, &sphere, &light, &position, &eyev, &normalv, 1.0);
        assert_eq!(result, color(1.6364, 1.6364, 1.6364));
    }
}

#[cfg(test)]
mod test_chapter_8_shadows {
    use super::*;

    use crate::sphere::*;

    #[test]
    fn lighting_with_the_surface_in_shadow() {
        let (m, position) = test_common::setup();
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = point_light(&point(0.0, 0.0, -10.0), &color(1.0, 1.0, 1.0));
        let light_intensity = 0.0;
        let sphere = sphere().unwrap();
        let result = m.lighting(&sphere, &light, &position, &eyev, &normalv, light_intensity);
        assert_eq!(result, color(0.1, 0.1, 0.1));
    }
}

#[cfg(test)]
mod test_chapter_10_material_pattern {
    #![allow(non_snake_case)]

    use super::*;

    use crate::sphere::*;

    #[test]
    fn lighting_with_a_pattern_applied() {
        let mut m = Material::default();
        let pattern = stripe_pattern(&color(1.0, 1.0, 1.0), &color(0.0, 0.0, 0.0)).unwrap();
        m.set_pattern(pattern);
        m.ambient = 1.0;
        m.diffuse = 0.0;
        m.specular = 0.0;
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = point_light(&point(0.0, 0.0, -10.0), &color(1.0, 1.0, 1.0));
        let sphere = sphere().unwrap();
        let c1 = lighting(
            &m,
            &sphere,
            &light,
            &point(0.9, 0.0, 0.0),
            &eyev,
            &normalv,
            1.0,
        );
        let c2 = lighting(
            &m,
            &sphere,
            &light,
            &point(1.1, 0.0, 0.0),
            &eyev,
            &normalv,
            1.0,
        );
        assert_eq!(c1, color(1.0, 1.0, 1.0));
        assert_eq!(c2, color(0.0, 0.0, 0.0));
    }
}

#[cfg(test)]
mod test_chapter_11_reflection {
    use super::*;

    #[test]
    fn reflectivity_for_the_default_material() {
        let m = material();
        assert_eq!(m.reflective, 0.0);
    }

    #[test]
    fn transparency_and_refractive_index_for_the_default_material() {
        let m = material();
        assert_eq!(m.transparency, 0.0);
        assert_eq!(m.refractive_index, 1.0);
    }
}
