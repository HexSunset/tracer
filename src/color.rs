use super::*;

pub fn write_color(color: &Color) {
    let r = (255.0 * color.x()) as u8;
    let g = (255.0 * color.y()) as u8;
    let b = (255.0 * color.z()) as u8;
    
    println!("{} {} {}", r, g, b);
}
