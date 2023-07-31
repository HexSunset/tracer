use super::*;

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let r = (255.0 * self.0.x()) as u8;
        let g = (255.0 * self.0.y()) as u8;
        let b = (255.0 * self.0.z()) as u8;

        write!(f, "{} {} {}", r, g, b)
    }
}

//pub fn write_color(color: &Color) {
//    let r = (255.0 * color.x()) as u8;
//    let g = (255.0 * color.y()) as u8;
//    let b = (255.0 * color.z()) as u8;
//
//    println!("{} {} {}", r, g, b);
//}
