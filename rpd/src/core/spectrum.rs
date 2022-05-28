use std::ops::{Add, Mul, Sub};
use crate::core::math::*;

pub type Spectrum = RGBSpectrum;

#[derive(Debug, Clone, Default)]
pub struct RGBSpectrum {
    pub rgb: Vector3d,
}

impl Mul for RGBSpectrum {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        return RGBSpectrum::new(
            Vector3d::new(
                self.rgb.x * rhs.rgb.x,
                self.rgb.y * rhs.rgb.y,
                self.rgb.z * rhs.rgb.z,
            )
        );
    }
}

impl Add for RGBSpectrum {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        return RGBSpectrum::new(
            Vector3d::new(
                self.rgb.x + rhs.rgb.x,
                self.rgb.y + rhs.rgb.y,
                self.rgb.z + rhs.rgb.z,
            )
        );
    }
}

impl Sub for RGBSpectrum {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        return RGBSpectrum::new(
            Vector3d::new(
                self.rgb.x - rhs.rgb.x,
                self.rgb.y - rhs.rgb.y,
                self.rgb.z - rhs.rgb.z,
            )
        );
    }
}

impl RGBSpectrum {
    pub fn new(rgb: Vector3d) -> RGBSpectrum {
        return RGBSpectrum {
            rgb: rgb
        };
    }
}