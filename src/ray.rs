use std::ops::Range;

use bevy_color::{Color, ColorToComponents, LinearRgba, Mix};
use bevy_math::{vec3, Dir3, NormedVectorSpace, Ray3d, Vec3};
use tracing::debug;

use crate::{hittable::Hittable, objects::Sphere, random::random_on_hemisphere};

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

    pub fn direction(&self) -> Dir3 {
        self.inner.direction
    }

    pub fn origin(&self) -> Vec3 {
        self.inner.origin
    }

    /// Given some normal, compares it to this ray.
    /// Returns
    pub fn facing_same_general_direction(&self, normal: Dir3) -> bool {
        self.direction().dot(normal.into()) > 0.0
    }

    /// A position some distance along the ray
    pub fn at(&self, t: f32) -> Vec3 {
        self.inner.get_point(t)
    }

    pub fn sky_color(&self) -> Color {
        let y = self.inner.direction.y;

        // Range [-1.0, 1.0] rescaled to [0.0, 1.0].
        // When looking down, we're looking more and more towards -1.0 (remapped to 0.0).
        // In that case we want white. So that's the start value.
        let a = (y + 1.0) * 0.5;

        let white = Color::WHITE;
        let blue: Color = LinearRgba::from_vec3(vec3(0.5, 0.7, 1.0)).into();

        white.mix(&blue, a)
    }

    pub fn world_color(&self, world: &dyn Hittable, range: Range<f32>) -> Color {
        match world.hit(self, range) {
            // hit: remap the colors of the surface normal
            Some(hit) => LinearRgba::from_vec3(0.5 * (Vec3::from(hit.normal) + Vec3::ONE)).into(),
            None => self.sky_color(),
        }
    }

    pub fn world_color_bounce(
        &self,
        world: &dyn Hittable,
        range: Range<f32>,
        bounce: usize,
    ) -> Color {
        // either exhaust the bounces (dark!)
        // or return sky color with less color proportional to # bounces

        if bounce == 0 {
            return Color::BLACK;
        }

        match world.hit(self, range.clone()) {
            Some(hit) => {
                let new_dir = random_on_hemisphere(hit.normal);
                (0.5 * Self::new(hit.point, new_dir)
                    .world_color_bounce(world, range, bounce - 1)
                    .to_linear())
                .into()
            }
            None => self.sky_color(),
        }
    }

    pub fn hit_sphere(&self, sphere: &Sphere) -> f32 {
        // We got (-b +- sqrt(b^2 - 4ac)) / 2a.
        // If we substitute b = -2h:
        // 2h +- sqrt(4h^2 - 4ac) / 2a = (2h +- 2 * sqrt(h^2 - ac)) / 2a =
        // = (h +- sqrt(h^2 - ac) / a
        // So then the discriminant is h^2 - ac.
        //
        // So if b = -2h = -2 * ray_dir.dot(-ray_origin + sphere_center)
        // then h = ray_dir.dot(-ray_origin + sphere_center)

        let d = self.inner.direction;
        let q = -self.inner.origin + sphere.center;

        let h = d.dot(q);

        let b = -2. * d.dot(q);
        let c = q.length_squared() - sphere.radius.powi(2);

        let discriminant = h.norm_squared() - c;

        if discriminant < 0.0 {
            -1.0
        } else {
            debug!("b: {b:.2}, discriminant: {discriminant:.2}");
            h - discriminant.sqrt()
        }
    }
}
