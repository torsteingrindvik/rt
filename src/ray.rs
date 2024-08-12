use bevy_color::{Color, ColorToComponents, LinearRgba, Mix};
use bevy_math::{vec3, Dir3, Ray3d, Vec3};

#[derive(Debug)]
pub struct Ray {
    inner: Ray3d,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Dir3) -> Self {
        Self {
            inner: Ray3d { origin, direction },
        }
    }

    pub fn color(&self) -> Color {
        let y = self.inner.direction.y;

        // range [0.0, 1.0]
        let a = (y + 1.0) * 0.5;

        let white = Color::WHITE;
        let blue: Color = LinearRgba::from_vec3(vec3(0.5, 0.7, 1.0)).into();

        white.mix(&blue, a)
    }

    pub fn hit_sphere(&self, sphere_center: Vec3, sphere_radius: f32) -> bool {
        let d = self.inner.direction;
        let phi = self.inner.origin - sphere_center;

        let a = d.dot(d.into());
        let b = -2. * d.dot(phi);
        let c = phi.dot(phi) - sphere_radius.powi(2);

        let discriminant = b.powi(2) - 4. * a * c;

        discriminant >= 0.0
    }
}
