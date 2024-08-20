use std::ops::Range;

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
