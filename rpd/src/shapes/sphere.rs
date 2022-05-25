use std::ops::Mul;
use nalgebra_glm::radians;
use crate::core::geometry::{Bounds3, Ray};
use crate::core::interaction::SurfaceInteraction;
use crate::core::math::{Point3d, Vector3d, *};
use crate::core::Res;
use crate::core::transform::Transform;
use crate::interface::shape::{Shape, ShapeData};

#[derive(Debug, Clone, Default)]
pub struct Sphere {
    pub radius: f64,
    pub shapeData: ShapeData,
}

impl Shape for Sphere {
    fn objectBound(&self) -> Bounds3 {
        return Bounds3 { pMin: Point3d::new(-self.radius, -self.radius, -self.radius), pMax: Point3d::new(self.radius, self.radius, self.radius) };
    }

    fn worldBound(&self) -> Bounds3 {
        let mut center = self.shapeData.objectToWorld.m.mul(Vector4d::new(0f64, 0f64, 0f64, 1f64));
        center = center / center.w;
        return Bounds3 { pMin: Point3d::new(center.x - self.radius, center.y - self.radius, center.z - self.radius), pMax: Point3d::new(center.x + self.radius, center.y + self.radius, center.z + self.radius) };
    }

    fn intersect(&self, ray: &Ray, testAlphaTexture: bool, t: &mut f64, isect: &mut SurfaceInteraction) -> Res<bool> {
        let mut transed_center = self.shapeData.objectToWorld.m.mul(Vector4d::new(0.0, 0.0, 0.0, 1.0));
        let a = ray.d.dot(&ray.d);
        let b = 2f64 * (ray.d.x * ray.o.x + ray.d.y * ray.o.y + ray.d.z * ray.o.z);
        let olen = ray.o.len() as f64;
        let c = olen.powf(2f64) - self.radius * self.radius;

        if let Ok((t0, t1)) = Quadratic(&a, &b, &c) {
            *t = t0;
            return Ok(true);
        }
        return Err(String::from("No intersection on this sphere"));
    }

    fn area(&self) -> f64 {
        return self.radius.powf(2f64) * 4f64 * std::f64::consts::PI;
    }
}

impl Sphere {
    pub fn new(objectToWorld: &Transform, r: f64) -> Sphere {
        return Sphere {
            radius: r,
            shapeData: ShapeData {
                objectToWorld: objectToWorld.clone(),
                worldToObject: Transform::new(&objectToWorld.inv),
                reverseOrientation: false,
                transformSwapHandedness: false,
            },
        };
    }
}