use crate::{color::*, float::*, light::*, tuple::*};

pub fn material() -> Material {
    Material::default()
}

pub fn lighting(
    material: &Material,
    light: &Light,
    point: &Point,
    eyev: &Vector,
    normalv: &Vector,
) -> Color {
    material.lighting(light, point, eyev, normalv)
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

    pub fn lighting(&self, light: &Light, point: &Point, eyev: &Vector, normalv: &Vector) -> Color {
        // Combine the surface color with the light's color/intensity
        let effective_color = self.color * light.intensity;

        // Find the direction to the light source
        let lightv = (light.position - point).normalize();

        // Compute the ambient contribution
        let ambient = effective_color * self.ambient;

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
            let reflectv = reflect(-lightv, normalv);
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
        ambient + diffuse + specular
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

    #[test]
    fn lighting_with_the_eye_between_the_light_and_the_surface() {
        let m = material();
        let position = point(0., 0., 0.);
        let eyev = vector(0., 0., -1.);
        let normalv = vector(0., 0., -1.);
        let light = point_light(point(0., 0., -10.), color(1., 1., 1.));
        let result = lighting(&m, &light, &position, &eyev, &normalv);
        assert_eq!(result, color(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighting_with_the_eye_between_the_light_and_surface_eye_offset_45_degrees() {
        let m = material();
        let position = point(0., 0., 0.);
        let eyev = vector(0., 2.0_f32.sqrt() / 2.0, -2.0_f32.sqrt() / 2.0);
        let normalv = vector(0., 0., -1.);
        let light = point_light(point(0., 0., -10.), color(1., 1., 1.));
        let result = lighting(&m, &light, &position, &eyev, &normalv);
        assert_eq!(result, color(1.0, 1.0, 1.0));
    }

    #[test]
    fn lighting_with_the_eye_opposite_surface_light_offset_45_degrees() {
        let m = material();
        let position = point(0., 0., 0.);
        let eyev = vector(0., 0., -1.);
        let normalv = vector(0., 0., -1.);
        let light = point_light(point(0., 10., -10.), color(1., 1., 1.));
        let result = lighting(&m, &light, &position, &eyev, &normalv);
        assert_eq!(result, color(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn lighting_with_eye_in_the_path_of_the_reflection_vector() {
        let m = material();
        let position = point(0., 0., 0.);
        let eyev = vector(0., -2.0_f32.sqrt() / 2.0, -2.0_f32.sqrt() / 2.0);
        let normalv = vector(0., 0., -1.);
        let light = point_light(point(0., 10., -10.), color(1., 1., 1.));
        let result = lighting(&m, &light, &position, &eyev, &normalv);
        assert_eq!(result, color(1.6364, 1.6364, 1.6364));
    }
}
