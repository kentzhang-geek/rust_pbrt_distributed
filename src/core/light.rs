use std::sync::Arc;
use crate::core::interaction::InteractionInterface;
use crate::core::math::Vector3d;
use super::Spectrum;

/// interface of light
/// # WIP
pub trait Light {
    fn power(&self)->Spectrum;
}

/// Arealight, which always use as emission material
pub trait AreaLight : Light{
    fn L(&self, intr : Arc<dyn InteractionInterface>, w : &Vector3d)->Spectrum;
}