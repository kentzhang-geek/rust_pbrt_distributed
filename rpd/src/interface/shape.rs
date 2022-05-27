use std::rc::Rc;
use std::sync::{Arc, Weak};
use crate::core::geometry::{Bounds3, Ray};
use crate::core::interaction::{InteractionInterface, SurfaceInteraction};
use crate::core::math::{Matrix44d, Point2f};
use crate::core::Res;
use crate::core::transform::Transform;

/// Shape interface declaration
pub trait Shape {
    fn objectBound(&self) -> Bounds3;
    fn worldBound(&self) -> Bounds3;
    fn intersect(self: &Self, ray: &Ray, testAlphaTexture: bool, t: &mut f64, isect: &mut SurfaceInteraction) -> Res<bool>;
    fn area(&self)->f64;
    fn try_method(self: Arc<Self>, isect : & mut SurfaceInteraction);
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
