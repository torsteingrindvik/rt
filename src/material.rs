use bevy_color::{Color, LinearRgba};
use bevy_math::Dir3;
use std::{fmt::Debug, ops::Deref, sync::Arc};

use crate::{hittable::Hit, random::random_on_sphere, ray::Ray};

#[derive(Debug, Clone)]
pub struct DynMaterial(Arc<Box<dyn Material>>);

impl Deref for DynMaterial {
    type Target = dyn Material;

    fn deref(&self) -> &Self::Target {
        &**self.0
    }
}

impl DynMaterial {
    pub fn new(material: impl Material + 'static) -> Self {
        Self(Arc::new(Box::new(material)))
    }
}

impl From<Lambertian> for DynMaterial {
    fn from(value: Lambertian) -> Self {
        Self::new(value)
    }
}

impl From<Metal> for DynMaterial {
    fn from(value: Metal) -> Self {
        Self::new(value)
    }
}

pub trait Material: Debug {
    /// Given a ray and a [`Hit`] by that ray,
    /// scatter by the material properties
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scattering>;
}

pub struct Scattering {
    /// The ray in the scatter direction
    pub ray: Ray,
    pub attenuation: Color,
}

#[derive(Debug)]
pub struct Lambertian {
    pub color: Color,
}

impl Lambertian {
    pub fn linear_rgb(red: f32, green: f32, blue: f32) -> Self {
        Self {
            color: LinearRgba::rgb(red, green, blue).into(),
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &Hit) -> Option<Scattering> {
        let scatter_dir = hit.normal.as_vec3() + random_on_sphere().as_vec3();

        let scattered = Ray::new(hit.point, scatter_dir);

        Some(Scattering {
            ray: scattered,
            attenuation: self.color,
        })
    }
}

#[derive(Debug)]
pub struct Metal {
    pub color: Color,
    pub fuzz: f32,
}

impl Metal {
    /// Create a metallic material with a given fuzz factor.
    /// The fuzz factor is clamped to the [0.0, 1.0] range.
    pub fn new(color: Color, fuzz: f32) -> Self {
        Self {
            color,
            fuzz: fuzz.clamp(0.0, 1.0),
        }
    }

    pub fn linear_rgb(red: f32, green: f32, blue: f32) -> Self {
        Self {
            color: LinearRgba::rgb(red, green, blue).into(),
            fuzz: 0.0,
        }
    }
}

// todo: glam 0.29 has a builtin reflect
trait Glam029 {
    fn reflect(&self, normal: Dir3) -> Dir3;
}

impl Glam029 for Dir3 {
    fn reflect(&self, normal: Dir3) -> Dir3 {
        let me_v3 = self.as_vec3();
        let n_v3 = normal.as_vec3();

        Dir3::new_unchecked((me_v3 - 2.0 * me_v3.dot(n_v3) * n_v3).normalize())
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scattering> {
        let scatter_dir = ray.direction().reflect(hit.normal);
        let fuzzed_dir = scatter_dir.as_vec3().normalize() + self.fuzz * random_on_sphere();

        if hit.normal.dot(fuzzed_dir).is_sign_positive() {
            let scattered = Ray::new(hit.point, fuzzed_dir);

            Some(Scattering {
                ray: scattered,
                attenuation: self.color,
            })
        } else {
            None
        }
    }
}
