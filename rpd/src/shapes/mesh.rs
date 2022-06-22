use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use crate::core::geometry::{Bounds3, Ray};
use crate::core::interaction::SurfaceInteraction;
use crate::core::math::Vector3d;
use crate::core::Res;
use crate::interface::shape::{Shape, ShapeData};

/// attribute should bind to vertex, so there are mostly 2 methods
#[derive(Debug)]
pub enum AttributeMappingMode {
    None = 0,
    ByVertex = 1,
    ByIndexes = 2,
}

/// only support triangle mesh
/// most used model structure
#[derive(Debug)]
pub struct Mesh {
    pub base : ShapeData,
    pub vertexs : Vec<Vector3d>,
    pub indecis : Vec<(i32, i32, i32)>,
    pub normalMappingMode : AttributeMappingMode,
    pub normal : Vec<Vector3d>,
}

impl Shape for Mesh {
    fn area(&self) -> f64 {
        todo!()
    }
    fn intersect(self: Arc<Self>, ray: &Ray, testAlphaTexture: bool, t: &mut f64, isect: &mut SurfaceInteraction) -> Res<bool> {
        todo!()
    }
    fn worldBound(&self) -> Bounds3 {
        todo!()
    }
    fn objectBound(self: &Self) -> Bounds3 {
        todo!()
    }
}
