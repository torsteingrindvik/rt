use bevy_color::palettes;
use bevy_color::{ColorToPacked, LinearRgba};
use bevy_math::{vec3, Dir3, Vec3};
use clap::{Parser, Subcommand};
use rt_one::ppm;
use rt_one::ray;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Writes the first PPM image seen in chapter 2.2 to "first.ppm"
    FirstPpm,

    /// Writes the white blue gradient from chapter 4.2 to "gradient.ppm"
    Gradient,

    /// The ray -> sphere hit in chapter 5.1
    RaySphere,
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli.command {
        Command::FirstPpm => first_ppm(),
        Command::Gradient => gradient(),
        Command::RaySphere => ray_sphere(),
    }
}

fn first_ppm() -> anyhow::Result<()> {
    let mut data = vec![];
    for row in 0..=255 {
        for col in 0..=255 {
            let color = LinearRgba::from_u8_array_no_alpha([col, row, 0]);
            data.extend(color.to_u8_array_no_alpha());
        }
    }

    ppm::write_pathlike(256, data, "image.ppm")
}

fn gradient() -> anyhow::Result<()> {
    let im_width = 400;

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

    let viewport_origin =
        cam_origin - vec3(0.0, 0.0, focal_length) - viewport_u / 2. - viewport_v / 2.;
    let pixel00_origin = viewport_origin + 0.5 * (du + dv);

    let mut data = vec![];

    for row in 0..im_height {
        for col in 0..im_width {
            let pixel = pixel00_origin + (row as f32 * dv) + (col as f32 * du);
            let dir = Dir3::new_unchecked((-cam_origin + pixel).normalize());
            let ray = ray::Ray::new(cam_origin, dir);

            let color = ray.color();
            data.extend(color.to_linear().to_u8_array_no_alpha());
        }
    }

    ppm::write_pathlike(im_height, data, "gradient.ppm")
}

fn ray_sphere() -> anyhow::Result<()> {
    let im_width = 400;

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
    let viewport_v = vec3(0.0, viewport_height, 0.0);

    let du = viewport_u / im_width as f32;
    let dv = viewport_v / im_height as f32;

    let viewport_origin =
        cam_origin + vec3(0.0, 0.0, focal_length) - viewport_u / 2. - viewport_v / 2.;
    let pixel00_origin = viewport_origin + 0.5 * (du + dv);

    let mut data = vec![];

    for row in 0..im_height {
        for col in 0..im_width {
            let pixel = pixel00_origin + (row as f32 * dv) + (col as f32 * du);
            let dir: Dir3 = Dir3::new_unchecked((-cam_origin + pixel).normalize());
            let ray = ray::Ray::new(cam_origin, dir);

            let color = if ray.hit_sphere(Vec3::new(1.1, 0.1, 2.0), 0.3) {
                palettes::tailwind::AMBER_100.into()
            } else {
                ray.color()
            };
            data.extend(color.to_linear().to_u8_array_no_alpha());
        }
    }

    ppm::write_pathlike(im_height, data, "ray_sphere.ppm")
}
