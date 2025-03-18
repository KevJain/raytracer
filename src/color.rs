use std::io::Write;
use std::io::Result;
use crate::geometry;

pub type Color = geometry::Vec3;

// Colour is a Vec3 with all values between 0.0 and 1.0 (not inclusive)
pub fn write_pixel<W: Write> (writer: &mut W, color: Color) -> Result<()> {
    let rbyte = (color.x * 255.999) as u8;
    let gbyte = (color.y * 255.999) as u8;
    let bbyte = (color.z * 255.999) as u8;
    writeln!(writer, "{} {} {}", rbyte, gbyte, bbyte)
}