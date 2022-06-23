use std::ops::Mul;
use std::sync::Arc;
use crate::core::geometry::{Bounds3, Ray};
use crate::core::interaction::SurfaceInteraction;
use crate::core::math::{Vector3d, CommonTools, Vector4d};
use crate::core::Res;
use crate::interface::shape::{Shape, ShapeData};

/// this triangle struct does not store data, only reference them, and provide interfaces for compute
#[derive(Debug)]
pub struct Triangle<'t> {
    pub base: &'t ShapeData,
    /// 3 points
    pub pa: &'t Vector3d,
    pub pb: &'t Vector3d,
    pub pc: &'t Vector3d,
    /// 3 normals
    pub na: &'t Vector3d,
    pub nb: &'t Vector3d,
    pub nc: &'t Vector3d,
}

const EPSILON: f64 = 0.00001f64;

impl Triangle<'_> {
    fn pointToWorld(&self, pt: &Vector3d) -> Vector3d {
        let mut p = Vector4d::new(pt.x, pt.y, pt.z, 1.0f64);
        let mut p = self.base.objectToWorld.m.mul(p);
        p = p / p.w;
        return p.xyz();
    }
    /// triangle ray intersection algorithm, from Moller, learned from GAMES 101
    /// return Barycentric point of triangle, or None when miss
    pub fn intersectByMollerAlgorithm(&self, r: &Ray) -> Option<Vector3d> {
        /// using Moller algorithm
        let mut E1 = self.pb - self.pa;
        let mut E2 = self.pc - self.pa;
        let mut S = r.o - self.pa;
        let mut S1 = r.d.cross(&E2);
        let mut S2 = S.cross(&E1);
        let mut s1_e1: f64 = S1.dot(&E1);
        if s1_e1 > -EPSILON {
            s1_e1 = s1_e1.abs().max(EPSILON);
        } else {
            s1_e1 = -s1_e1.abs().max(EPSILON);
        }
        let mut triangle_barycentric = Vector3d::new(
            S2.dot(&E2) / s1_e1,
            S1.dot(&S) / s1_e1,
            S2.dot(&r.d) / s1_e1,
        );
        if triangle_barycentric.x >= EPSILON
            && triangle_barycentric.y >= -EPSILON
            && triangle_barycentric.z >= -EPSILON
            && (1f64 - triangle_barycentric.y - triangle_barycentric.z) >= -EPSILON {
            return Some(triangle_barycentric);
        }
        return None;
    }
    pub fn rayHit(&self, ray: &mut Ray, testAlphaTexture: bool, t: &mut f64, isect: &mut SurfaceInteraction) -> Res<bool> {
        if let Some(bt) = self.intersectByMollerAlgorithm(ray) {
            let pt = self.pa * bt.x + self.pb * bt.y + self.pc * bt.z;
            let mut res = (pt - ray.o).norm();
            if ray.tmax > res {
                *t = res;
                ray.tmax = res;
                isect.interaction.p = pt;
                isect.interaction.n = self.na * bt.x + self.nb * bt.y + self.nc * bt.z;
                isect.shading.n = isect.interaction.n;
            }
            return Ok(true);
        }
        return Ok(false);
    }
}

impl Shape for Triangle<'_> {
    fn objectBound(self: &Self) -> Bounds3 {
        let mut bnd = Bounds3 {
            pMin: Vector3d::MinPerComponent(self.pa, self.pb),
            pMax: Vector3d::MaxPerComponent(self.pa, self.pb),
        };
        return Bounds3::addOnePoint(&bnd, *self.pc);
    }
    fn worldBound(&self) -> Bounds3 {
        let a = self.pointToWorld(self.pa);
        let b = self.pointToWorld(self.pb);
        let c = self.pointToWorld(self.pc);
        let mut bnd = Bounds3 {
            pMin: Vector3d::MinPerComponent(&a, &b),
            pMax: Vector3d::MaxPerComponent(&a, &b),
        };
        return Bounds3::addOnePoint(&bnd, c);
    }
    fn area(&self) -> f64 {
        let a = (self.pa - self.pc).cross(&(self.pb - self.pc));
        return a.norm();
    }
    fn intersect(self: Arc<Self>, ray: &mut Ray, testAlphaTexture: bool, t: &mut f64, isect: &mut SurfaceInteraction) -> Res<bool> {
        if let Some(bt) = self.intersectByMollerAlgorithm(ray) {
            let pt = self.pa * bt.x + self.pb * bt.y + self.pc * bt.z;
            let mut res = (pt - ray.o).norm();
            if ray.tmax > res {
                *t = res;
                ray.tmax = res;
                isect.interaction.p = pt;
                isect.interaction.n = self.na * bt.x + self.nb * bt.y + self.nc * bt.z;
                isect.shading.n = isect.interaction.n;
            }
            return Ok(true);
        }
        return Ok(false);
    }
}