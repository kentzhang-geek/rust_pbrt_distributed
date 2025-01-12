use std::sync::Arc;
use super::math::*;
use super::geometry::*;

/// The interfaces of Cameras
pub trait Camera {
    fn generate_ray(&self, sample:Arc<Box<CameraSample>>)->Option<CameraRayResult>;
}

pub struct CameraSample {
    film : Vector2d,
    lens : Vector2d,
}

pub struct CameraRayResult {
    pub ray : Arc<Box<Ray>>,
    pub radian: f64,
}