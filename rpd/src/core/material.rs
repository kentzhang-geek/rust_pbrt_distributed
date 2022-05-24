use std::sync::Arc;
use crate::core::interaction::SurfaceInteraction;

pub trait Material {
    fn ComputeScatteringFunctions(&self, si : Arc<Box<SurfaceInteraction>>);
}