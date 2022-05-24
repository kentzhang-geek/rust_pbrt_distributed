use std::cmp::{min, max};
use crate::core::Res;
use super::math::*;

/// An abstraction of ray
#[derive(Debug, Clone)]
pub struct Ray {
    pub o: Point3d,
    pub d: Vector3d,
    pub tmax: f64,
}

impl Ray {
    /// Compute a point at length t
    pub fn at(&self, t: f64) -> Point3d {
        return self.o + self.d * t;
    }
    /// construct a new ray
    pub fn new(origin: Point3d, direction: Vector3d) -> Ray {
        return Ray {
            o: origin,
            d: direction,
            tmax: Ray::default_tmax(),
        };
    }
    /// default tMax
    pub fn default_tmax() -> f64 {
        return 100000f64;
    }
}

/// Axis aligned bounding box
#[derive(Debug, Clone, Default)]
pub struct Bounds3 {
    pub pMin: Point3d,
    pub pMax: Point3d,
}

impl Bounds3 {
    /// compute 2 coarse intersection point on this AABB
    pub fn IntersectP(&self, r: &Ray) -> Res<(f64, f64)> {
        let mut t0 = 0.0f64;
        let mut t1 = r.tmax;

        for i in 0..2 {
            let invRayDir = 1.0f64 / r.d[i];
            let mut tNear = (self.pMin[i] - r.o[i]) * invRayDir;
            let mut tFar = (self.pMax[i] - r.o[i]) * invRayDir;

            if tNear > tFar {
                (tNear, tFar) = (tFar, tNear);
            }

            // Update _tFar_ to ensure robust ray--bounds intersection
            // tFar *= 1 + 2 * gamma(3);
            t0 = if tNear > t0 {tNear} else { t0 };
            t1 = if tFar < t1 {tFar} else { t1 };

            if t0 > t1 {
                return Err(String::from("Intersection not exists"));
            }
        }

        return Ok((t0, t1));
    }
}