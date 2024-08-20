use bevy_math::{Dir3, NormedVectorSpace, Vec3};
use tracing::debug;

use crate::hittable::{Hit, Hittable};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Hittable for Sphere {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_range: std::ops::Range<f32>,
    ) -> Option<crate::hittable::Hit> {
        // We got (-b +- sqrt(b^2 - 4ac)) / 2a.
        // If we substitute b = -2h:
        // 2h +- sqrt(4h^2 - 4ac) / 2a = (2h +- 2 * sqrt(h^2 - ac)) / 2a =
        // = (h +- sqrt(h^2 - ac) / a
        // So then the discriminant is h^2 - ac.
        //
        // So if b = -2h = -2 * ray_dir.dot(-ray_origin + sphere_center)
        // then h = ray_dir.dot(-ray_origin + sphere_center)

        let d = ray.direction();
        let q = -ray.origin() + self.center;

        let h = d.dot(q);

        let b = -2. * d.dot(q);
        let c = q.length_squared() - self.radius.powi(2);

        let discriminant = h.norm_squared() - c;

        if discriminant < 0.0 {
            None
        } else {
            debug!("b: {b:.2}, discriminant: {discriminant:.2}");
            let discr_sqrt = discriminant.sqrt();

            let t1 = h - discr_sqrt;
            let t2 = h + discr_sqrt;

            let t = if t_range.contains(&t1) {
                t1
            } else if t_range.contains(&t2) {
                t2
            } else {
                return None;
            };

            let at = ray.at(t);
            let outward_normal = Dir3::new_unchecked((-self.center + at).normalize());
            let front_face = !ray.facing_same_general_direction(outward_normal);
            let normal = if front_face {
                outward_normal
            } else {
                -outward_normal
            };

            Some(Hit {
                point: at,
                normal,
                distance: t,
                front_face,
            })
        }
    }
}
