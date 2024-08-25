use bevy_math::{vec3, Vec3};

#[allow(dead_code)]
pub struct Setup {
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
}

impl Setup {
    pub fn new() -> Self {
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
        }
    }
}
