use super::*;

// TODO, chapter 6.3
trait Hittable {}

#[derive(Clone, Copy, PartialEq)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3) -> Ray {
	Self {
	    origin: orig,
	    direction: dir,
	}
    }

    pub fn origin(&self) -> Point3 {
	self.origin
    }

    pub fn at(&self, t: f64) -> Point3 {
	self.origin() + self.direction() * t
    }

    pub fn direction(&self) -> Vec3 {
	self.direction
    }

    // This is a temporary blue -> white gradient
    pub fn ray_color(&self) -> Color {
	match self.hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5) {
	    Some(t) => {
		let n = (self.at(t) - Vec3::new(0.0, 0.0, -1.0)).normalize();
		return Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0) * 0.5;
	    },
	    None => {},
	}
	let unit_dir = self.direction().normalize();
	let t = 0.5 * (unit_dir.y() + 1.0);
	return Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t;
    }

    pub fn hit_sphere(&self, center: Point3, radius: f64) -> Option<f64> {
	let oc = self.origin() - center;
	let a = self.direction().len_sqr();
	let half_b = oc.dot(self.direction);
	let c = oc.len_sqr() - radius * radius;

	let discriminant = half_b * half_b - a * c;
	if discriminant < 0.0 {
	    None
	} else {
	    Some((-half_b - discriminant.sqrt()) / a)
	}
    }
}
