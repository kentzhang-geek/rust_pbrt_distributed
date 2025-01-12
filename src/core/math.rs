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

pub trait CommonTools<T> {
    fn One() -> T;
    fn Zero() -> T;
    fn MinPerComponent(v1: &T, v2: &T) -> T;
    fn MaxPerComponent(v1: &T, v2: &T) -> T;
}

impl CommonTools<Vector3d> for Vector3d {
    fn One() -> Vector3d {
        return vector![1.0f64, 1.0f64, 1.0f64];
    }
    fn Zero() -> Vector3d {
        return vector![0f64, 0f64, 0f64];
    }

    fn MinPerComponent(v1: &Vector3d, v2: &Vector3d) -> Vector3d {
        return Vector3d::new(
            v1.x.min(v2.x),
            v1.y.min(v2.y),
            v1.z.min(v2.z),
        );
    }

    fn MaxPerComponent(v1: &Vector3d, v2: &Vector3d) -> Vector3d {
        return Vector3d::new(
            v1.x.max(v2.x),
            v1.y.max(v2.y),
            v1.z.max(v2.z),
        );
    }
}

impl CommonTools<Vector3f> for Vector3f {
    fn One() -> Vector3f {
        return vector![1f32,1f32,1f32];
    }

    fn Zero() -> Vector3f {
        return vector![0f32,0f32,0f32];
    }

    fn MinPerComponent(v1: &Vector3f, v2: &Vector3f) -> Vector3f {
        return Vector3f::new(
            v1.x.min(v2.x),
            v1.y.min(v2.y),
            v1.z.min(v2.z),
        );
    }

    fn MaxPerComponent(v1: &Vector3f, v2: &Vector3f) -> Vector3f {
        return Vector3f::new(
            v1.x.max(v2.x),
            v1.y.max(v2.y),
            v1.z.max(v2.z),
        );
    }
}

impl CommonTools<Vector2f> for Vector2f {
    fn One() -> Vector2f { return Vector2f::new(1f32, 1f32); }

    fn Zero() -> Vector2f { return Vector2f::new(0f32, 0f32); }

    fn MinPerComponent(v1: &Vector2f, v2: &Vector2f) -> Vector2f {
        return Vector2f::new(
            v1.x.min(v2.x),
            v1.y.min(v2.y),
        );
    }

    fn MaxPerComponent(v1: &Vector2f, v2: &Vector2f) -> Vector2f {
        return Vector2f::new(
            v1.x.max(v2.x),
            v1.y.max(v2.y),
        );
    }
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
    } else {
        return Ok((r1, r2));
    }
}

pub mod octahedron_texture {
    use crate::core::math::{Vector2d, Vector2i, Vector3d};

    fn signNotZero(k:f64) ->f64 {
        if k >= 0.0f64 {
            1.0f64
        } else {
            -1.0f64
        }
    }

    fn signNotZero2(v : Vector2d) -> Vector2d {
        return Vector2d::new(signNotZero(v.x), signNotZero(v.y));
    }

    fn multPerComponent(v1 : Vector2d, v2 : Vector2d) -> Vector2d {
        return Vector2d::new(v1.x * v2.x, v1.y * v2.y);
    }

    pub fn octEncode(v : Vector3d) -> Vector2d {
        let l1norm = v.x.abs() + v.y.abs() + v.z.abs();
        let mut result = v.xy() * (1.0f64 / l1norm);
        if v.z < 0.0 {
            result = multPerComponent(Vector2d::new(1.0f64, 1.0f64) - result.yx().abs(), signNotZero2(result.xy()));
        }
        return result;
    }

    pub fn octDecode(o : Vector2d) -> Vector3d {
        let mut v = Vector3d::new(o.x, o.y, 1.0f64 - o.x.abs() - o.y.abs());
        if v.z < 0.0 {
            let mut newvxy = multPerComponent(Vector2d::new(1.0, 1.0) - v.yx().abs(), signNotZero2(v.xy()));
            v = Vector3d::new(newvxy.x, newvxy.y, v.z);
        }
        return v.normalize();
    }

    pub fn fromIntCoordToOctFloatCoord(pixCoord : Vector2i, probeOctahedronSize : i32) -> Vector2d {
        let probeOctahedronSize = probeOctahedronSize as f64;
        let mut ret = Vector2d::new(pixCoord.x as f64 + 0.5f64, pixCoord.y as f64 + 0.5f64);
        ret = multPerComponent(ret, Vector2d::new(1.0f64 / probeOctahedronSize, 1.0f64 / probeOctahedronSize));
        return multPerComponent(ret, Vector2d::new(2.0f64, 2.0f64)) - Vector2d::new(1.0f64, 1.0f64);
    }
}
