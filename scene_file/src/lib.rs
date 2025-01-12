pub mod scene;

use crate::scene::sf::*;

impl Matrix44dT {
    pub fn unit_mat()->Matrix44dT{
        Matrix44dT{
            rows: [
                Vec4dT{
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                    w: 0.0
                },
                Vec4dT{
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                    w: 0.0
                },
                Vec4dT{
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                    w: 0.0
                },
                Vec4dT{
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                    w: 1.0
                },
            ]
        }
    }
}
