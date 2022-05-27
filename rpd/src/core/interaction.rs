use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;
use std::sync::{Arc, Weak};
use crate::core::geometry::Ray;
use crate::core::math::*;
use crate::interface::shape::Shape;
use crate::shapes::sphere::Sphere;

/// Basic data storage of intersection
#[derive(Debug, Clone, Default)]
pub struct InteractionBase {
    /// position of intersection
    pub p: Vector3d,
    /// normal of intersection
    pub n: Vector3d,
    /// when interaction happend
    pub time: f64,
}

/// Interface of interaction
/// mostly on surface
/// # WIP
pub trait InteractionInterface {
    fn IsSurfaceInteraction(&self) -> bool;
    fn SpawnRay(&self, d : &Vector3d)->Ray;
}

/// surface shading property , when surface intersection happend
#[derive(Debug, Clone, Default)]
pub struct SurfaceShading {
    pub n : Vector3d,
}

/// surface intersection data
#[derive(Clone, Default)]
pub struct SurfaceInteraction
{
    pub interaction: InteractionBase,
    pub uv : Vector2f,
    pub shape : Option<Arc<dyn Shape>>,
    pub shading : SurfaceShading,
}
