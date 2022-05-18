use nalgebra;
use super::math::*;


/// Coordinate framework related to World Space, may consider level heriachy structure in future
#[derive(Debug, Clone, Default)]
pub struct CoordinateSystem {
    pub X: Vector3d,
    pub Y: Vector3d,
    pub Z: Vector3d,
}

impl CoordinateSystem {
    pub fn New(x:Vector3d, y:Vector3d, z:Vector3d)->CoordinateSystem {
        CoordinateSystem{
            X:x, Y:y, Z:z
        }
    }
}