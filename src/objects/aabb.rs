use std::cmp::Ordering;
use std::mem::swap;

use crate::ray::Ray;
use crate::vec3::Point3;

use crate::objects::bounding_box::BoundingBox;

pub struct AABB {
    low: Point3,
    high: Point3,
}

impl AABB {
    #[inline(always)]
    pub fn new(low: Point3, high: Point3) -> AABB {
        AABB { low, high }
    }

    #[inline(always)]
    pub fn get_low(&self) -> Point3 {
        self.low
    }

    #[inline(always)]
    pub fn get_high(&self) -> Point3 {
        self.high
    }

    #[inline(always)]
    pub fn surrounding_box(bb0: AABB, bb1: AABB) -> AABB {
        let small = Point3::new(
            bb0.low.get_x().min(bb1.low.get_x()),
            bb0.low.get_y().min(bb1.low.get_y()),
            bb0.low.get_z().min(bb1.low.get_z()),
        );
        let big = Point3::new(
            bb0.high.get_x().max(bb1.high.get_x()),
            bb0.high.get_y().max(bb1.high.get_y()),
            bb0.high.get_z().max(bb1.high.get_z()),
        );
        AABB {
            low: small,
            high: big,
        }
    }
}

impl BoundingBox for AABB {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
        let mut t_min = t_min;
        let mut t_max = t_max;
        for i in 0..3 {
            let inv_dir = 1.0 / ray.get_direction()[i];
            let mut t0 = (self.low[i] - ray.get_origin()[i]) * inv_dir;
            let mut t1 = (self.high[i] - ray.get_origin()[i]) * inv_dir;
            if inv_dir < 0.0 {
                swap(&mut t0, &mut t1);
            }
            t_min = t0.max(t_min);
            t_max = t1.min(t_max);
            if t_max <= t_min {
                return false;
            }
        }
        true
    }

    #[inline(always)]
    fn get_lower(&self) -> Point3 {
        self.get_low()
    }

    #[inline(always)]
    fn get_upper(&self) -> Point3 {
        self.get_high()
    }

    #[inline(always)]
    fn cmp_axis(&self, other: &Box<dyn BoundingBox>, axis: usize) -> Ordering {
        self.get_lower()[axis].partial_cmp(&other.get_lower()[axis]).unwrap()
    }
}
