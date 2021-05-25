use super::vec3::{Point3, Vec3};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
    time: f64,
}

impl Ray {
    #[inline(always)]
    pub fn new(origin: Point3, direction: Vec3, time: f64) -> Ray {
        Ray {
            origin,
            direction,
            time,
        }
    }

    #[inline(always)]
    pub fn get_origin(self) -> Point3 {
        self.origin
    }

    #[inline(always)]
    pub fn get_direction(self) -> Vec3 {
        self.direction
    }

    #[inline(always)]
    pub fn get_time(self) -> f64 {
        self.time
    }

    #[inline(always)]
    pub fn at(self, t: f64) -> Vec3 {
        self.origin + (t * self.direction)
    }
}
