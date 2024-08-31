use bevy_color::{palettes, Color};
use bevy_color::{ColorToPacked, LinearRgba};
use bevy_math::{Dir3, Vec3};
use clap::{Parser, Subcommand};
use rt_one::camera::Camera;
use rt_one::hittable::Hittables;
use rt_one::objects::Sphere;
use rt_one::ppm;
use rt_one::ray;
use tracing::info;

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

    /// The ray -> sphere hit with normal colors in chapter 6.1
    RaySphereNormal,

    /// A world of hittables. Shows normal sphere and big "earth" sphere. Chapter 6.7
    Hittables,

    /// Adding anti-aliasing. Chapter 8.2
    AntiAliasing,

    /// Diffuse sphere. Chapter 9.2
    FirstDiffuse,

    /// Diffuse sphere without shadow acne. Chapter 9.3
    DiffuseNoAcne,

    /// Using Lambertian scattering instead of uniform. Chapter 9.4
    Lambertian,
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli.command {
        Command::FirstPpm => first_ppm(),
        Command::Gradient => gradient(),
        Command::RaySphere => ray_sphere(),
        Command::RaySphereNormal => ray_sphere_normal_colors(),
        Command::Hittables => hittables(),
        Command::AntiAliasing => anti_aliasing(),
        Command::FirstDiffuse => first_diffuse(),
        Command::DiffuseNoAcne => diffuse_no_acne(),
        Command::Lambertian => lambertian(),
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
    let camera = Camera::new();

    let mut data = vec![];

    for row in 0..camera.im_height {
        for col in 0..camera.im_width {
            let pixel = camera.pixel00_origin + (row as f32 * camera.dv) + (col as f32 * camera.du);
            let dir = Dir3::new_unchecked((-camera.cam_origin + pixel).normalize());
            let ray = ray::Ray::new(camera.cam_origin, dir);

            let color = camera.sky_color(&ray);
            data.extend(color.to_linear().to_u8_array_no_alpha());
        }
    }

    ppm::write_pathlike(camera.im_height, data, "gradient.ppm")
}

fn ray_sphere() -> anyhow::Result<()> {
    let camera = Camera::new();

    let mut data = vec![];

    for row in 0..camera.im_height {
        for col in 0..camera.im_width {
            let pixel = camera.pixel00_origin + (row as f32 * camera.dv) + (col as f32 * camera.du);
            let dir = Dir3::new_unchecked((-camera.cam_origin + pixel).normalize());
            let ray = ray::Ray::new(camera.cam_origin, dir);

            let sphere = Sphere {
                center: Vec3::new(0.0, 0.0, -1.0),
                radius: 0.5,
            };

            let color: Color = if ray.hit_sphere(&sphere) >= 0.0 {
                palettes::tailwind::RED_500.into()
            } else {
                camera.sky_color(&ray)
            };

            data.extend(color.to_linear().to_u8_array_no_alpha());
        }
    }

    ppm::write_pathlike(camera.im_height, data, "ray_sphere.ppm")
}

fn ray_sphere_normal_colors() -> anyhow::Result<()> {
    let c = Camera::new();

    let mut data = vec![];

    for row in 0..c.im_height {
        for col in 0..c.im_width {
            let pixel = c.pixel00_origin + (row as f32 * c.dv) + (col as f32 * c.du);
            // Unit direction from camera to pixel
            let dir: Dir3 = Dir3::new_unchecked((-c.cam_origin + pixel).normalize());

            if row == 0 && col == 0 {
                info!(
                    "First row, first col, dir: {dir:#?}. Pixel origin: {:#?}",
                    c.pixel00_origin
                );
            }

            if row == (c.im_height - 1) && col == (c.im_width - 1) {
                info!("Last row, last col, dir: {dir:#?}");
            }

            let ray = ray::Ray::new(c.cam_origin, dir);

            let sphere = Sphere {
                center: Vec3::new(0.0, 0.0, -1.0),
                radius: 0.5,
            };
            let ray_hit_t = ray.hit_sphere(&sphere);

            let color = if ray_hit_t > 0.0 {
                let hit_pos = ray.at(ray_hit_t);
                let mut n = (-sphere.center + hit_pos).normalize();

                // Each component has possible range [-1.0, 1.0], so remap
                n += Vec3::ONE;
                n /= 2.0;

                LinearRgba::new(n.x, n.y, n.z, 1.0)
            } else {
                c.sky_color(&ray).to_linear()
            };

            data.extend(color.to_u8_array_no_alpha());
        }
    }

    ppm::write_pathlike(c.im_height, data, "ray_sphere_normal.ppm")
}

fn hittables() -> anyhow::Result<()> {
    let mut world = Hittables::default();

    world.add(Sphere {
        center: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5,
    });
    world.add(Sphere {
        center: Vec3::new(0.0, -100.5, -1.0),
        radius: 100.0,
    });

    Camera::new().render(&world, "hittable.ppm")
}

fn anti_aliasing() -> anyhow::Result<()> {
    let mut world = Hittables::default();

    world.add(Sphere {
        center: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5,
    });
    world.add(Sphere {
        center: Vec3::new(0.0, -100.5, -1.0),
        radius: 100.0,
    });

    Camera::with_samples_per_pixel(10).render(&world, "anti_aliasing.ppm")
}

fn first_diffuse() -> anyhow::Result<()> {
    let mut world = Hittables::default();

    world.add(Sphere {
        center: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5,
    });
    world.add(Sphere {
        center: Vec3::new(0.0, -100.5, -1.0),
        radius: 100.0,
    });

    let mut camera = Camera::with_samples_per_pixel(10);
    camera.bounce = 50;
    camera.render(&world, "first_diffuse.ppm")
}

fn diffuse_no_acne() -> anyhow::Result<()> {
    let mut world = Hittables::default();

    world.add(Sphere {
        center: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5,
    });
    world.add(Sphere {
        center: Vec3::new(0.0, -100.5, -1.0),
        radius: 100.0,
    });

    let mut camera = Camera::with_samples_per_pixel(10);
    camera.bounce = 50;
    camera.min_dist = 0.001;
    camera.render(&world, "diffuse_no_acne.ppm")
}

fn lambertian() -> anyhow::Result<()> {
    let mut world = Hittables::default();

    world.add(Sphere {
        center: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5,
    });
    world.add(Sphere {
        center: Vec3::new(0.0, -100.5, -1.0),
        radius: 100.0,
    });

    let mut camera = Camera::with_samples_per_pixel(10);
    camera.bounce = 50;
    camera.min_dist = 0.001;
    camera.render(&world, "diffuse_no_acne.ppm")
}
