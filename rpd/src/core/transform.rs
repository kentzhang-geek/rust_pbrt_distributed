use scene_file::common::{Matrix44dT, Vec4dT};
use crate::interface::io::Pack;
use super::math::*;

#[derive(Debug, Clone, Default)]
pub struct Transform {
    pub m : Matrix44d,
    pub inv : Matrix44d,
}

impl Transform {
    pub fn new(m : & Matrix44d)->Transform {
        return Transform {
            m : m.clone(),
            inv : m.try_inverse().unwrap()
        }
    }
}

impl Pack<Matrix44dT> for Transform {
    fn pack(&self) -> Box<Matrix44dT> {
        return Box::new(Matrix44dT{
            idx: [
                Vec4dT{
                    x: self.m.m11,
                    y: self.m.m12,
                    z: self.m.m13,
                    w: self.m.m14
                },
                Vec4dT{
                    x: self.m.m21,
                    y: self.m.m22,
                    z: self.m.m23,
                    w: self.m.m24
                },
                Vec4dT{
                    x: self.m.m31,
                    y: self.m.m32,
                    z: self.m.m33,
                    w: self.m.m34
                },
                Vec4dT{
                    x: self.m.m41,
                    y: self.m.m42,
                    z: self.m.m43,
                    w: self.m.m44
                },
            ]
        });
    }
}