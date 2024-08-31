use bevy_math::{Dir3, NormedVectorSpace, Ray3d, Vec3};
use tracing::debug;

use crate::objects::Sphere;

#[derive(Debug)]
pub struct Ray {
    inner: Ray3d,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            inner: Ray3d {
                origin,
                direction: Dir3::new_unchecked(direction.normalize()),
            },
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
