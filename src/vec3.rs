use std::convert::From;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Neg, Sub, SubAssign};

use rand::{thread_rng, Rng};

use super::color::Color;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

pub type Point3 = Vec3;

impl Vec3 {
    #[inline(always)]
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    #[inline(always)]
    pub fn random(min: f64, max: f64) -> Vec3 {
        Vec3::random_with_gen(&mut thread_rng(), min, max)
    }

    #[inline(always)]
    pub fn random_with_gen<R: Rng + ?Sized>(rng: &mut R, min: f64, max: f64) -> Vec3 {
        Vec3 {
            x: rng.gen_range(min..max),
            y: rng.gen_range(min..max),
            z: rng.gen_range(min..max),
        }
    }

    #[inline(always)]
    pub fn random_in_unit_sphere() -> Vec3 {
        Vec3::random_in_unit_sphere_with_gen(&mut thread_rng())
    }

    pub fn random_in_unit_sphere_with_gen<R: Rng + ?Sized>(rng: &mut R) -> Vec3 {
        loop {
            let p = Vec3::random_with_gen(rng, -1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    #[inline(always)]
    pub fn random_in_unit_disk() -> Vec3 {
        Vec3::random_in_unit_disk_with_gen(&mut thread_rng())
    }

    pub fn random_in_unit_disk_with_gen<R: Rng + ?Sized>(rng: &mut R) -> Vec3 {
        loop {
            let p = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    #[inline(always)]
    pub fn dot(&self, rhs: &Vec3) -> f64 {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }

    #[inline(always)]
    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: (self.y * rhs.z) - (self.z * rhs.y),
            y: (self.z * rhs.x) - (self.x * rhs.z),
            z: (self.x * rhs.y) - (self.y * rhs.x),
        }
    }

    #[inline(always)]
    pub fn get_x(&self) -> f64 {
        self.x
    }

    #[inline(always)]
    pub fn get_y(&self) -> f64 {
        self.y
    }

    #[inline(always)]
    pub fn get_z(&self) -> f64 {
        self.z
    }

    #[inline(always)]
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    #[inline(always)]
    pub fn length_squared(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    #[inline(always)]
    pub fn unit_vector(&self) -> Vec3 {
        return *self / self.length();
    }

    #[inline(always)]
    pub fn scale_in_range(&mut self, scale: f64, min: f64, max: f64) {
        self.x = (self.x * scale).sqrt().clamp(min, max);
        self.y = (self.y * scale).sqrt().clamp(min, max);
        self.z = (self.z * scale).sqrt().clamp(min, max);
    }

    #[inline(always)]
    pub fn near_zero(&self) -> bool {
        self.x.abs() < 1e-8 && self.y.abs() < 1e-8 && self.z.abs() < 1e-8
    }

    #[inline(always)]
    pub fn reflect(&self, normal: &Vec3) -> Vec3 {
        *self - (2.0 * self.dot(normal) * *normal)
    }

    pub fn refract(&self, normal: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = self.neg().dot(normal).min(1.0);
        let r_out_perp = etai_over_etat * (*self + (cos_theta * *normal));
        let r_out_parallel = (1.0 - r_out_perp.length_squared()).abs().sqrt().neg() * *normal;
        r_out_perp + r_out_parallel
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        *self = Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        };
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, divisor: f64) -> Vec3 {
        Vec3 {
            x: self.x / divisor,
            y: self.y / divisor,
            z: self.z / divisor,
        }
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, divisor: f64) {
        *self = Vec3 {
            x: self.x / divisor,
            y: self.y / divisor,
            z: self.z / divisor,
        };
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Vec3) {
        *self = Vec3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        };
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("index {} out of bounds for Vec3", idx),
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, factor: f64) -> Vec3 {
        Vec3 {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, factor: f64) {
        *self = Vec3 {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor,
        };
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        *self = Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        };
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Vec3 {
        -1.0 * self
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        *self = Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        };
    }
}

impl From<Color> for Vec3 {
    fn from(c: Color) -> Self {
        Vec3::new(
            c.get_red() as f64 / 255.0,
            c.get_green() as f64 / 255.0,
            c.get_blue() as f64 / 255.0,
        )
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
        assert_eq!(p1.dot(&p2), expected);
    }

    #[test]
    fn test_cross_product() {
        let p1 = Vec3::new(35.0, 43.0, 55.0);
        let p2 = Vec3::new(43.0, 67.0, 83.0);
        let expected = Vec3::new(
            (43.0 * 83.0) - (55.0 * 67.0),
            (55.0 * 43.0) - (35.0 * 83.0),
            (35.0 * 67.0) - (43.0 * 43.0),
        );
        assert_eq!(p1.cross(&p2), expected);
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
