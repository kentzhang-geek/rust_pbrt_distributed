use std::any::Any;
use std::rc::Rc;
use std::sync::{Arc, Weak};
use crate::core::geometry::{Bounds3, Ray};
use crate::core::interaction::{InteractionInterface, SurfaceInteraction};
use crate::core::math::{Matrix44d, *};
use crate::core::Res;
use crate::core::transform::Transform;

/// Shape interface declaration
pub trait Shape  {
    fn objectBound(self: &Self) -> Bounds3;
    fn worldBound(&self) -> Bounds3;
    fn intersect(self: Arc<Self>, ray: & mut Ray, testAlphaTexture: bool, t: &mut f64, isect: &mut SurfaceInteraction) -> Res<bool>;
    fn area(&self)->f64;
    fn as_any(&self)->& dyn Any;
    // Sample a point on the surface of the shape and return the PDF with
    // respect to area on the surface.
    // fn Sample(&self, u : & Point2f, pdf : & mut f64)->dyn InteractionInterface;
}

/// basic shape data
#[derive(Debug, Clone, Default)]
pub struct ShapeData {
    pub objectToWorld : Transform,
    pub worldToObject : Transform,
    pub reverseOrientation : bool,
    pub transformSwapHandedness : bool,
}
