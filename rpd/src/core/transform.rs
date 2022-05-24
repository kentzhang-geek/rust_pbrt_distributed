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