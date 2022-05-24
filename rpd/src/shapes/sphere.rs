use std::ops::Mul;
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
        todo!()
    }

    fn area(&self) -> f64 {
        todo!()
    }
}

impl Sphere {
    pub fn new(objectToWorld : &Transform, r : f64)-> Sphere {
        return Sphere {
            radius: r,
            shapeData: ShapeData{
                objectToWorld : objectToWorld.clone(),
                worldToObject: Transform::new(&objectToWorld.inv),
                reverseOrientation: false,
                transformSwapHandedness: false
            }
        };
    }
}