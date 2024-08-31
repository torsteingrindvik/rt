use std::path::Path;

use bevy_color::{ColorToPacked, LinearRgba, Srgba};
use bevy_math::{vec3, Dir3, Vec2, Vec3, VectorSpace};
use rand::random;
use tracing::info;

use crate::{hittable::Hittable, ppm, ray};

#[allow(dead_code)]
pub struct Camera {
    pub im_width: usize,
    pub im_height: usize,
    pub aspect_ratio: f32,

    pub viewport_height: f32,
    pub viewport_width: f32,
    pub viewport_u: Vec3,
    pub viewport_v: Vec3,
    pub du: Vec3,
    pub dv: Vec3,
    pub viewport_origin: Vec3,
    pub pixel00_origin: Vec3,

    pub focal_length: f32,
    pub cam_origin: Vec3,

    pub samples_per_pixel: usize,
    pub bounce: usize,
    pub min_dist: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self::with_samples_per_pixel(1)
    }

    pub fn with_samples_per_pixel(samples: usize) -> Self {
        let im_width = 600;

        // the width/height relationship
        let mut aspect_ratio = 16. / 9.;

        let im_height = ((im_width as f32 / aspect_ratio) as usize).max(1);

        // recalc since height might have been modified
        aspect_ratio = im_width as f32 / im_height as f32;

        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;

        let focal_length = 1.0;
        let cam_origin = Vec3::ZERO;

        let viewport_u = vec3(viewport_width, 0.0, 0.0);
        let viewport_v = vec3(0.0, -viewport_height, 0.0);

        let du = viewport_u / im_width as f32;
        let dv = viewport_v / im_height as f32;

        // Viewport is at cam origin, then focal length in negative Z (forward) dir,
        // then we offset by the viewport horizontally and vertically since we'll iter over
        // that in parts.
        let viewport_origin =
            cam_origin - vec3(0.0, 0.0, focal_length) - viewport_u / 2. - viewport_v / 2.;

        // Make sure pixels are located in the middle of grid
        let pixel00_origin = viewport_origin + 0.5 * (du + dv);

        Self {
            im_width,
            im_height,
            aspect_ratio,
            viewport_height,
            viewport_width,
            viewport_u,
            viewport_v,
            du,
            dv,
            viewport_origin,
            pixel00_origin,
            focal_length,
            cam_origin,
            samples_per_pixel: samples,
            bounce: 0,
            min_dist: 0.0,
        }
    }

    // Range is +- 0.5 on both axes
    fn sample_unit_square() -> Vec2 {
        let r = || random::<f32>() - 1.;
        Vec2::new(r(), r())
    }

    fn get_ray(&self, row: usize, col: usize) -> ray::Ray {
        let pixel = self.pixel00_origin + (row as f32 * self.dv) + (col as f32 * self.du);

        let perturb = Self::sample_unit_square();

        let mut pixel = pixel + perturb.x * self.du;
        pixel += perturb.y * self.dv;

        // Unit direction from camera to pixel
        let dir: Dir3 = Dir3::new_unchecked((-self.cam_origin + pixel).normalize());
        ray::Ray::new(self.cam_origin, dir)
    }

    pub fn render(
        &self,
        world: &dyn Hittable,
        output_file: impl AsRef<Path>,
    ) -> anyhow::Result<()> {
        let mut data = vec![];

        for row in 0..self.im_height {
            for col in 0..self.im_width {
                let mut color: LinearRgba = LinearRgba::ZERO;

                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(row, col);

                    let max_dist = 10_000_000.0;

                    if self.bounce > 0 {
                        color += ray
                            .world_color_bounce(world, self.min_dist..max_dist, self.bounce)
                            .to_linear();
                    } else {
                        color += ray.world_color(world, self.min_dist..max_dist).to_linear();
                    }
                }

                color /= self.samples_per_pixel as f32;

                data.extend(color.to_u8_array_no_alpha());
            }
        }

        ppm::write_pathlike(self.im_height, data, output_file)?;

        Ok(())
    }
}
