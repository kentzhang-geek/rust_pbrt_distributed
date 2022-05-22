use std::sync::Arc;
use super::math::*;
use super::geometry::*;

/// The interfaces of Cameras
pub trait Camera {
    fn generate_ray(&self, sample:Arc<Box<CameraSample>>)->Option<CameraRayResult>;
}

pub struct CameraSample {
    film : Point2d,
    lens : Point2d,
}

pub struct CameraRayResult {
    pub ray : Arc<Box<Ray>>,
    pub radian: f64,
}