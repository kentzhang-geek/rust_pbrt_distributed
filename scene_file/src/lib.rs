pub mod common;
pub mod bvh_accel;
pub mod mesh_primitive;
pub mod scene;
pub mod camera;
pub mod texture;

impl common::Matrix44dT {
    pub fn unit_mat()->common::Matrix44dT{
        common::Matrix44dT{
            idx: [
                common::Vec4dT{
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                    w: 0.0
                },
                common::Vec4dT{
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                    w: 0.0
                },
                common::Vec4dT{
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                    w: 0.0
                },
                common::Vec4dT{
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                    w: 1.0
                },
            ]
        }
    }
}
