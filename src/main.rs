pub mod vec3;
pub mod color;
pub mod ray;

pub use vec3::Color;
pub use vec3::Point3;
pub use vec3::Vec3;
pub use ray::Ray;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

fn main() {
    // Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0, 0.0, focal_length);

    // Render
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for y in (0..IMAGE_HEIGHT).rev() {
        eprint!("[i] Writing line {}/{}\r", IMAGE_HEIGHT - y, IMAGE_HEIGHT);

        for x in 0..IMAGE_WIDTH {
	    let u: f64 = x as f64 / (IMAGE_WIDTH - 1) as f64;
	    let v: f64 = y as f64 / (IMAGE_HEIGHT - 1) as f64;

	    let r = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v - origin);
	    let pixel = r.ray_color();
	    color::write_color(&pixel)
        }
    }
    eprintln!("\n[+] Done!")
}
