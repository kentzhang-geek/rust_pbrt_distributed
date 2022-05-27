use std::fmt::Debug;
use std::rc::Rc;
use std::sync::{Arc, Weak};
use crate::core::geometry::Ray;
use crate::core::math::{Point2f, Point3d, Vector3d};
use crate::interface::shape::Shape;
use crate::shapes::sphere::Sphere;

#[derive(Debug, Clone, Default)]
pub struct InteractionBase {
    pub p: Point3d,
    pub n: Vector3d,
    pub time: f64,
}

pub trait InteractionInterface {
    fn IsSurfaceInteraction(&self) -> bool;
    fn SpawnRay(&self, d : &Vector3d)->Ray;
}

#[derive(Debug, Clone, Default)]
pub struct SurfaceShading {
    pub n : Vector3d,
}

#[derive(Clone, Default)]
pub struct SurfaceInteraction
{
    pub interaction: InteractionBase,
    pub uv : Point2f,
    pub shape : Option<Weak<dyn Shape>>,
    pub shading : SurfaceShading,
}
