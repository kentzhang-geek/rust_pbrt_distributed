use nalgebra;
use nalgebra::{dvector, OPoint};
use nalgebra::vector;
use nalgebra_glm::sqrt;
use crate::core::Res;

pub type Vector2d = nalgebra::Vector2<f64>;
pub type Vector2f = nalgebra::Vector2<f32>;
pub type Vector2i = nalgebra::Vector2<i32>;

pub type Vector3d = nalgebra::Vector3<f64>;
pub type Vector3f = nalgebra::Vector3<f32>;

pub type Vector4d = nalgebra::Vector4<f64>;
pub type Vector4f = nalgebra::Vector4<f32>;

pub type Matrix44d = nalgebra::Matrix4<f64>;
pub type Matrix44f = nalgebra::Matrix4<f32>;

pub trait UsualDatas<T> {
    fn One() -> T;
    fn Zero() -> T;
}

impl UsualDatas<Vector3d> for Vector3d {
    fn One() -> Vector3d {
        return vector![1.0f64, 1.0f64, 1.0f64];
    }
    fn Zero() -> Vector3d {
        return vector![0f64, 0f64, 0f64];
    }
}

impl UsualDatas<Vector3f> for Vector3f {
    fn One() -> Vector3f {
        return vector![1f32,1f32,1f32];
    }

    fn Zero() -> Vector3f {
        return vector![0f32,0f32,0f32];
    }
}

impl UsualDatas<Vector2f> for Vector2f {
    fn One() -> Vector2f { return Vector2f::new(1f32, 1f32); }

    fn Zero() -> Vector2f { return Vector2f::new(0f32, 0f32); }
}

pub fn Quadratic(&a: &f64, &b: &f64, &c: &f64) -> Res<(f64, f64)> {
    let mut check = b * b - 4f64 * a * c;
    if check < 0f64 {
        return Err(String::from("Solution of this Quadratic not exists"));
    }
    check = (check as f64).sqrt();
    let r1 = (-b + check) / (2f64 * a);
    let r2 = (-b - check) / (2f64 * a);
    if r1 > r2 {
        return Ok((r2, r1));
    }
    else {
        return Ok((r1, r2));
    }
}