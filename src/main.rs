pub mod color;
pub mod ray;
pub mod vec3;

use std::io::Write;

pub use ray::Ray;
pub use vec3::Color;
pub use vec3::Point3;
pub use vec3::Vec3;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 1080;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

fn main() {
    use std::env::args;
    use std::process::exit;
    if args().len() != 2 {
        eprintln!("USAGE: tracer FILENAME");
        exit(1);
    }

    let out_file_name = args().nth(1).unwrap();
    use std::io::BufWriter;
    let mut file = BufWriter::new(std::fs::File::create(out_file_name).unwrap());

    // Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // Render
    writeln!(file, "P3").unwrap();
    writeln!(file, "{} {}", IMAGE_WIDTH, IMAGE_HEIGHT).unwrap();
    writeln!(file, "255").unwrap();

    // Progress reporter
    let mut debug_output = BufWriter::new(std::io::stderr());

    for y in (0..IMAGE_HEIGHT).rev() {
        write!(
            debug_output,
            "[i] Writing line {}/{}\r",
            IMAGE_HEIGHT - y,
            IMAGE_HEIGHT
        )
        .unwrap();

        for x in 0..IMAGE_WIDTH {
            let u: f64 = x as f64 / (IMAGE_WIDTH - 1) as f64;
            let v: f64 = y as f64 / (IMAGE_HEIGHT - 1) as f64;

            let r = Ray::new(
                origin,
                lower_left_corner + horizontal * u + vertical * v - origin,
            );
            let pixel = r.ray_color();
            writeln!(file, "{}", pixel).unwrap();
        }
    }

    writeln!(debug_output, "\r[+] Done! Wrote {} lines", IMAGE_HEIGHT).unwrap();
}
