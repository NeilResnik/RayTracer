use std::convert::TryFrom;
use std::io::Write;

use super::vec3::Vec3;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Color {
    red: u8,
    blue: u8,
    green: u8
}

impl Color {
    pub fn new(red: u8, blue: u8, green: u8) -> Color {
        Color { red, blue, green }
    }

    pub fn write_color<W: Write>(&self, mut writer: W) -> std::io::Result<()>{
        writeln!(&mut writer, "{} {} {}", self.red, self.blue, self.green)
    }

    pub fn get_red(&self) -> u8 {
        self.red
    }

    pub fn get_blue(&self) -> u8 {
        self.blue
    }

    pub fn get_green(&self) -> u8 {
        self.green
    }
}

impl TryFrom<Vec3> for Color {
    type Error = ();

    fn try_from(v: Vec3) -> Result<Self, Self::Error> {
        let vx = v.get_x();
        let vy = v.get_y();
        let vz = v.get_z();
        if vx >= 0.0 && vx <= 1.0 && vy >= 0.0 && vy <= 1.0 && vz >= 0.0 && vz <= 1.0 {
            Ok(Color::new((vx * 255.999) as u8, (vy * 255.999) as u8, (vz * 255.999) as u8))
        } else {
            Err(())
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_color() {
        let mut output = Vec::new();
        let c1 = Color::new(35, 43, 55);
        c1.write_color(&mut output).expect("Failed to write");
        let actual = String::from_utf8(output).expect("Not utf-8");
        let expected = String::from("35 43 55\n");
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_try_from_vec3() {
        let v = Vec3::new(35.0 / 255.0, 68.0 / 255.0, 127.0 / 255.0);
        let expected = Ok(Color::new(35, 68, 127));
        assert_eq!(Color::try_from(v), expected);
    }

    #[test]
    fn test_try_from_vec3_error() {
        let v = Vec3::new(1.2, 45.5, 23.4);
        assert_eq!(Color::try_from(v), Err(()));
    }
}
