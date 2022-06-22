use std::borrow::BorrowMut;
use std::ops::{Deref, Mul};
use std::rc::Rc;
use std::sync::{Arc, Mutex, Weak};
use nalgebra::{Translation, Vector3};
use nalgebra_glm::radians;
use crate::core::geometry::{Bounds3, Ray};
use crate::core::interaction::SurfaceInteraction;
use crate::core::math::{Vector3d, *};
use crate::core::Res;
use crate::core::tools::PrintSelf;
use crate::core::transform::Transform;
use crate::interface::shape::{Shape, ShapeData};

#[derive(Debug, Clone, Default)]
pub struct Sphere {
    pub radius: f64,
    pub shapeData: ShapeData,
    pub weak_self: Option<Weak<Self>>,
}

impl Shape for Sphere {
    fn objectBound(self: &Self) -> Bounds3 {
        return Bounds3 { pMin: Vector3d::new(-self.radius, -self.radius, -self.radius), pMax: Vector3d::new(self.radius, self.radius, self.radius) };
    }

    fn worldBound(&self) -> Bounds3 {
        let mut center = self.shapeData.objectToWorld.m.mul(Vector4d::new(0f64, 0f64, 0f64, 1f64));
        center = center / center.w;
        return Bounds3 { pMin: Vector3d::new(center.x - self.radius, center.y - self.radius, center.z - self.radius), pMax: Vector3d::new(center.x + self.radius, center.y + self.radius, center.z + self.radius) };
    }

    fn intersect(self: Arc<Self>, r: & mut Ray, testAlphaTexture: bool, t: &mut f64, isect: &mut SurfaceInteraction) -> Res<bool> {
        let newo = self.shapeData.worldToObject.m.mul(Vector4d::new(r.o.x, r.o.y, r.o.z, 1.0f64));
        let newd = self.shapeData.worldToObject.m.mul(Vector4d::new(r.d.x, r.d.y, r.d.z, 0.0f64));
        let ray = Ray::new(Vector3d::new(newo.x, newo.y, newo.z), Vector3d::new(newd.x, newd.y, newd.z));
        let a = ray.d.x * ray.d.x + ray.d.y * ray.d.y + ray.d.z * ray.d.z;
        let b = 2f64 * (ray.d.x * ray.o.x + ray.d.y * ray.o.y + ray.d.z * ray.o.z);
        let olen2 = ray.o.x * ray.o.x + ray.o.y * ray.o.y + ray.o.z * ray.o.z;
        let c = olen2 - self.radius * self.radius;

        if let Ok((t0, t1)) = Quadratic(&a, &b, &c) {
            if t0 > 0f64 {
                *t = t0;
            } else {
                *t = t1;
            }
            // record intersection data
            let p = r.at(*t);
            isect.interaction.n = (p - self.center()).normalize();
            isect.interaction.p = p;
            isect.shading.n = isect.interaction.n;
            isect.shape = Some(self.clone());
            r.tmax = *t;
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
        let ret = Sphere {
            radius: r,
            weak_self: None,
            shapeData: ShapeData {
                objectToWorld: objectToWorld.clone(),
                worldToObject: Transform::new(&objectToWorld.inv),
                reverseOrientation: false,
                transformSwapHandedness: false,
            },
        };
        return ret;
    }
    pub fn newOnlyMove(center: Vector3d, r: f64) -> Sphere {
        let m: Matrix44d = Matrix44d::from(Translation::from(center));
        // ::new(center.x, center.y, center.z);
        let ret = Sphere {
            radius: r,
            weak_self: None,
            shapeData: ShapeData {
                objectToWorld: Transform::new(&m),
                worldToObject: Transform::new(&m.try_inverse().unwrap()),
                reverseOrientation: false,
                transformSwapHandedness: false,
            },
        };
        return ret;
    }
    pub fn center(&self) -> Vector3d {
        let c = self.shapeData.objectToWorld.m.mul(Vector4d::new(0f64, 0f64, 0f64, 1f64));
        return c.xyz() / c.w;
    }
}