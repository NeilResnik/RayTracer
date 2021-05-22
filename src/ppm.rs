use std::io::{Result, Write};

use super::color::Color;

pub fn create_ppm<W: Write>(mut writer: W, image: Vec<Vec<Color>>) -> Result<()> {
    writeln!(writer, "P3")?;
    writeln!(writer, "{} {}", image[0].len(), image.len())?; // TODO?
    writeln!(writer, "255")?;
    for row in image {
        for pixel in row {
            pixel.write_color(&mut writer)?;
        }
    }
    Ok(())
}
