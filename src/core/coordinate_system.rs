use nalgebra;
use super::math::*;


/// Coordinate framework related to World Space, may consider level heriachy structure in future
#[derive(Debug, Clone)]
pub struct CoordinateSystem {
    pub X: Vector3d,
    pub Y: Vector3d,
    pub Z: Vector3d,
}

/// interfaces of CoordinateSystem
impl CoordinateSystem {
    /// Construct a new CoordinateSystem
    pub fn New(x:Vector3d, y:Vector3d, z:Vector3d)->CoordinateSystem {
        CoordinateSystem{
            X:x, Y:y, Z:z
        }
    }
}

impl Default for CoordinateSystem {
    fn default() -> Self {
        return CoordinateSystem::New(
            Vector3d::new(1f64, 0f64, 0f64),
            Vector3d::new(0f64, 1f64, 0f64),
            Vector3d::new(0f64, 0f64, 1f64),
        );
    }
}