use bevy_math::{Dir3, ShapeSample};

pub fn random_on_hemisphere(normal: Dir3) -> Dir3 {
    let mut rng = rand::thread_rng();
    let unit_sphere = bevy_math::prelude::Sphere::new(0.5).sample_boundary(&mut rng);

    let r = if unit_sphere.dot(*normal) > 0.0 {
        unit_sphere
    } else {
        -unit_sphere
    };

    Dir3::new(r).expect("unit sphere boundary should have unit length")
}
