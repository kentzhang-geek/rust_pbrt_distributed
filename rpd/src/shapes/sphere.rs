use std::borrow::BorrowMut;
use std::ops::{Deref, Mul};
use std::rc::Rc;
use std::sync::{Arc, Mutex, Weak};
use nalgebra_glm::radians;
use crate::core::geometry::{Bounds3, Ray};
use crate::core::interaction::SurfaceInteraction;
use crate::core::math::{Point3d, Vector3d, *};
use crate::core::Res;
use crate::core::tools::PrintSelf;
use crate::core::transform::Transform;
use crate::interface::shape::{Shape, ShapeData};

#[derive(Debug, Clone, Default)]
pub struct Sphere {
    pub radius: f64,
    pub shapeData: ShapeData,
    pub weak_self : Option<Weak<Self>>
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

    fn intersect(self: &Self, r: &Ray, testAlphaTexture: bool, t: &mut f64, isect: &mut SurfaceInteraction) -> Res<bool> {
        let newo = self.shapeData.worldToObject.m.mul(Vector4d::new(r.o.x, r.o.y, r.o.z, 1.0f64));
        let newd = self.shapeData.worldToObject.m.mul(Vector4d::new(r.d.x, r.d.y, r.d.z, 0.0f64));
        let ray = Ray::new(Point3d::new(newo.x, newo.y, newo.z), Vector3d::new(newd.x, newd.y, newd.z));
        let a = ray.d.x * ray.d.x + ray.d.y * ray.d.y + ray.d.z * ray.d.z;
        let b = 2f64 * (ray.d.x * ray.o.x + ray.d.y * ray.o.y + ray.d.z * ray.o.z);
        let olen2 = ray.o.x * ray.o.x + ray.o.y * ray.o.y + ray.o.z * ray.o.z;
        let c = olen2 - self.radius * self.radius;

        if let Ok((t0, t1)) = Quadratic(&a, &b, &c) {
            if t0 > 0f64 {
                *t = t0;
                // TODO : intersection fulfillment
                // isect.shape = Arc::downgrade(self);
            } else {
                *t = t1;
            }
            return Ok(true);
        }
        return Err(String::from("No intersection on this sphere"));
    }


    fn area(&self) -> f64 {
        return self.radius.powf(2f64) * 4f64 * std::f64::consts::PI;
    }

    fn try_method(self: Arc<Self>, isect: &mut SurfaceInteraction) {
        isect.shape = Some(Arc::<Sphere>::downgrade(&self));
    }


}

impl Sphere {
    pub fn new(objectToWorld: &Transform, r: f64) -> Arc<Sphere> {
        let ret = Arc::new(Sphere {
            radius: r,
            weak_self : None,
            shapeData: ShapeData {
                objectToWorld: objectToWorld.clone(),
                worldToObject: Transform::new(&objectToWorld.inv),
                reverseOrientation: false,
                transformSwapHandedness: false,
            },
        });
        return ret;
    }
}