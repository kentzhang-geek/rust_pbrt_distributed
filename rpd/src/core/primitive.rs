use std::sync::Arc;
use crate::core::geometry::{Bounds3, Ray};
use crate::core::interaction::SurfaceInteraction;
use crate::core::material::Material;
use crate::core::Res;

pub trait Primitive {
    fn worldBound(&self)->Bounds3;
    // fn Intersect(&self, ray : &Ray)->Res<Arc<Box<SurfaceInteraction>>>;
    fn HasIntersect(&self, ray : &Ray)->bool;
    fn GetMaterial(&self)->dyn Material;
}