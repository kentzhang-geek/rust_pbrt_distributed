use std::cmp::{min, max};
use nalgebra::SimdPartialOrd;
use crate::core::Res;
use super::math::*;

/// An abstraction of ray
#[derive(Debug, Clone)]
pub struct Ray {
    pub o: Vector3d,
    pub d: Vector3d,
    pub tmax: f64,
}

impl Ray {
    /// Compute a point at length t
    pub fn at(&self, t: f64) -> Vector3d {
        return self.o + self.d * t;
    }
    /// construct a new ray
    pub fn new(origin: Vector3d, direction: Vector3d) -> Ray {
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
#[derive(Debug, Clone, Default, Copy)]
pub struct Bounds3 {
    pub pMin: Vector3d,
    pub pMax: Vector3d,
}

impl Bounds3 {
    /// compute 2 coarse intersection point on this AABB
    pub fn intersect(&self, r: &Ray) -> Res<(f64, f64)> {
        let mut t0 = 0.0f64;
        let mut t1 = r.tmax;

        for i in 0..3 {
            let invRayDir = 1.0f64 / r.d[i];
            let mut tNear = (self.pMin[i] - r.o[i]) * invRayDir;
            let mut tFar = (self.pMax[i] - r.o[i]) * invRayDir;

            if tNear > tFar {
                (tNear, tFar) = (tFar, tNear);
            }

            // Update _tFar_ to ensure robust ray--bounds intersection
            // tFar *= 1 + 2 * gamma(3);
            t0 = if tNear > t0 { tNear } else { t0 };
            t1 = if tFar < t1 { tFar } else { t1 };

            if t0 > t1 {
                return Err(String::from("Intersection not exists"));
            }
        }

        return Ok((t0, t1));
    }

    /// get center position of this bound
    pub fn center(&self) -> Vector3d {
        return (self.pMin + self.pMax) / 2.0f64;
    }

    /// include 1 point in this bounds, return a big one
    pub fn addOnePoint(b: &Bounds3, pt: Vector3d) -> Bounds3 {
        return Bounds3 {
            pMin: Vector3d::MinPerComponent(&b.pMin, &pt),
            pMax: Vector3d::MaxPerComponent(&b.pMax, &pt),
        };
    }

    /// combine 2 bounds to a big one
    pub fn union(b1: &Bounds3, b2: &Bounds3) -> Bounds3 {
        return Bounds3 {
            pMin: Vector3d::MinPerComponent(&b1.pMin, &b2.pMin),
            pMax: Vector3d::MaxPerComponent(&b1.pMax, &b2.pMax),
        };
    }

    /// compute surface area
    pub fn surfaceArea(&self) -> f64 {
        let extent = self.pMax - self.pMin;
        return 2.0f64 * (
            extent.x * extent.y +
                extent.x * extent.z +
                extent.y * extent.z
        );
    }

    /// compute surface area with a minimum extent value to avoid zero surface area, if there is any division below may lead to errors
    pub fn protectedSurfaceArea(&self) -> f64 {
        let mut extent = self.pMax - self.pMin;
        let v = 0.0001f64;
        extent = Vector3d::MaxPerComponent(&extent, &Vector3d::new(v, v, v));
        return 2.0f64 * (
            extent.x * extent.y +
                extent.x * extent.z +
                extent.y * extent.z
        );
    }
}