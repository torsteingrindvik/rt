use std::{ops::Range, sync::Arc};

use bevy_math::{Dir3, Vec3};

use crate::ray::Ray;

#[derive(Debug)]
pub struct Hit {
    pub point: Vec3,

    /// The normal "against" the ray.
    /// In other words: Have the ray physically hit some object.
    /// The normal will point outwards from this physical hit point.
    ///
    /// An outside ray hitting a football will have a normal pointing out into the world.
    ///
    /// An inside ray e.g. originating within a hot air balloon will have the normal point back into
    /// the gaseous parts.
    pub normal: Dir3,

    /// If true the normal is pointing outwards (e.g. out from the football, out from the hot air balloon).
    /// Else will point inwards (e.g. into the pressurized part of the football).
    pub front_face: bool,

    /// Distance on the ray
    pub distance: f32,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_range: Range<f32>) -> Option<Hit>;
}

pub struct Hittables {
    pub objects: Vec<Arc<Box<dyn Hittable>>>,
}

impl Hittables {
    pub fn add(&mut self, object: impl Hittable + 'static) {
        self.objects.push(Arc::new(Box::new(object)));
    }
}

impl Hittable for Hittables {
    fn hit(&self, ray: &Ray, t_range: Range<f32>) -> Option<Hit> {
        let mut range = t_range;
        let mut closest_hit = None;

        for object in self.objects.iter() {
            if let Some(hit) = object.hit(ray, range.clone()) {
                // We passed in a range [close, far). Since there was a hit,
                // we shouldn't consider any hits beyond that since that would be
                // behind the current hit.
                // Therefore we shrink the far to be defined by this new hit.
                range.end = hit.distance;

                closest_hit = Some(hit);
            }
        }

        closest_hit
    }
}
