use std::convert::From;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use super::color::Color;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z}
    }

    pub fn dot(self, rhs: Vec3) -> f64 {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }

    pub fn cross(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: (self.y * rhs.z) - (self.z * rhs.y),
            y: (self.z * rhs.x) - (self.x * rhs.z),
            z: (self.x * rhs.y) - (self.y * rhs.x)
        }
    }

    pub fn get_x(self) -> f64 {
        self.x
    }

    pub fn get_y(self) -> f64 {
        self.y
    }

    pub fn get_z(self) -> f64 {
        self.z
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        *self = Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        };
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, divisor: f64) -> Vec3 {
        Vec3 {
            x: self.x / divisor,
            y: self.y / divisor,
            z: self.z / divisor
        }
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, divisor: f64) {
        *self = Vec3 {
            x: self.x / divisor,
            y: self.y / divisor,
            z: self.z / divisor
        };
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Vec3) {
        *self = Vec3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z
        };
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, factor: f64) -> Vec3 {
        Vec3 {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, factor: f64) {
        *self = Vec3 {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor
        };
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        *self = Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z
        };
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        *self = Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        };
    }
}

impl From<Color> for Vec3 {
    fn from(c: Color) -> Self {
        Vec3::new(c.get_red() as f64 / 255.0,
                  c.get_blue() as f64 / 255.0,
                  c.get_green() as f64 / 255.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dot_product() {
        let p1 = Vec3::new(35.0, 43.0, 55.0);
        let p2 = Vec3::new(43.0, 67.0, 83.0);
        let expected = (35.0 * 43.0) + (43.0 * 67.0) + (55.0 * 83.0);
        assert_eq!(p1.dot(p2), expected);
    }

    #[test]
    fn test_cross_product() {
        let p1 = Vec3::new(35.0, 43.0, 55.0);
        let p2 = Vec3::new(43.0, 67.0, 83.0);
        let expected = Vec3::new((43.0 * 83.0) - (55.0 * 67.0),
                                   (55.0 * 43.0) - (35.0 * 83.0),
                                   (35.0 * 67.0) - (43.0 * 43.0));
        assert_eq!(p1.cross(p2), expected);
    }

    #[test]
    fn test_from_color() {
        let c1 = Color::new(35, 55, 245);
        let expected = Vec3::new(35.0 / 255.0, 55.0 / 255.0, 245.0 / 255.0);
        assert_eq!(Vec3::from(c1), expected);
    }

    #[test]
    fn test_point3_add() {
        let p1 = Vec3::new(35.0, 43.0, 55.0);
        let p2 = Vec3::new(43.0, 67.0, 83.0);
        let expected = Vec3::new(35.0 + 43.0, 43.0 + 67.0, 55.0 + 83.0);
        assert_eq!(p1 + p2, expected);
    }

    #[test]
    fn test_point3_add_assign() {
        let mut p1 = Vec3::new(35.0, 43.0, 55.0);
        let p2 = Vec3::new(43.0, 67.0, 83.0);
        let expected = Vec3::new(35.0 + 43.0, 43.0 + 67.0, 55.0 + 83.0);
        p1 += p2;
        assert_eq!(p1, expected);
    }

    #[test]
    fn test_point3_div() {
        let p1 = Vec3::new(35.0, 43.0, 55.0);
        let p2 = Vec3::new(43.0, 67.0, 83.0);
        let expected = Vec3::new(35.0 / 43.0, 43.0 / 67.0, 55.0 / 83.0);
        assert_eq!(p1 / p2, expected);
    }

    #[test]
    fn test_point3_div_float() {
        let p1 = Vec3::new(35.0, 43.0, 55.0);
        let expected = Vec3::new(35.0 / 25.0, 43.0 / 25.0, 55.0 / 25.0);
        assert_eq!(p1 / 25.0, expected);
    }

    #[test]
    fn test_point3_div_assign() {
        let mut p1 = Vec3::new(35.0, 43.0, 55.0);
        let p2 = Vec3::new(43.0, 67.0, 83.0);
        let expected = Vec3::new(35.0 / 43.0, 43.0 / 67.0, 55.0 / 83.0);
        p1 /= p2;
        assert_eq!(p1, expected);
    }

    #[test]
    fn test_point3_div_assign_float() {
        let mut p1 = Vec3::new(35.0, 43.0, 55.0);
        let expected = Vec3::new(35.0 / 25.0, 43.0 / 25.0, 55.0 / 25.0);
        p1 /= 25.0;
        assert_eq!(p1, expected);
    }

    #[test]
    fn test_point3_mul() {
        let p1 = Vec3::new(35.0, 43.0, 55.0);
        let p2 = Vec3::new(43.0, 67.0, 83.0);
        let expected = Vec3::new(35.0 * 43.0, 43.0 * 67.0, 55.0 * 83.0);
        assert_eq!(p1 * p2, expected);
    }

    #[test]
    fn test_point3_mul_float() {
        let p1 = Vec3::new(35.0, 43.0, 55.0);
        let expected = Vec3::new(35.0 * 25.0, 43.0 * 25.0, 55.0 * 25.0);
        assert_eq!(p1 * 25.0, expected);
    }

    #[test]
    fn test_point3_mul_assign() {
        let mut p1 = Vec3::new(35.0, 43.0, 55.0);
        let p2 = Vec3::new(43.0, 67.0, 83.0);
        let expected = Vec3::new(35.0 * 43.0, 43.0 * 67.0, 55.0 * 83.0);
        p1 *= p2;
        assert_eq!(p1, expected);
    }

    #[test]
    fn test_point3_mul_assign_float() {
        let mut p1 = Vec3::new(35.0, 43.0, 55.0);
        let expected = Vec3::new(35.0 * 25.0, 43.0 * 25.0, 55.0 * 25.0);
        p1 *= 25.0;
        assert_eq!(p1, expected);
    }

    #[test]
    fn test_point3_sub() {
        let p1 = Vec3::new(43.0, 67.0, 83.0);
        let p2 = Vec3::new(35.0, 43.0, 55.0);
        let expected = Vec3::new(43.0 - 35.0, 67.0 - 43.0, 83.0 - 55.0);
        assert_eq!(p1 - p2, expected);
    }

    #[test]
    fn test_point3_sub_assign() {
        let mut p1 = Vec3::new(43.0, 67.0, 83.0);
        let p2 = Vec3::new(35.0, 43.0, 55.0);
        let expected = Vec3::new(43.0 - 35.0, 67.0 - 43.0, 83.0 - 55.0);
        p1 -= p2;
        assert_eq!(p1, expected);
    }
}
