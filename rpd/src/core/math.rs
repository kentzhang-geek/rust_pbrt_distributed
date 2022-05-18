use nalgebra;
use nalgebra::{dvector, OPoint};
use nalgebra::vector;

pub type Vector3d = nalgebra::Vector3<f64>;
pub type Vector3f = nalgebra::Vector3<f32>;

pub type Point3d = nalgebra::Point3<f64>;
pub type Point3f = nalgebra::Point3<f32>;

pub type Vector3 = Vector3d;
pub type Point3 = Point3d;

pub trait UsualDatas<T> {
    fn One() -> T;
    fn Zero() -> T;
}

impl UsualDatas<Vector3> for Vector3 {
    fn One() -> Vector3 {
        return vector![1.0f64, 1.0f64, 1.0f64];
    }
    fn Zero() -> Vector3 {
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

impl UsualDatas<Point3d> for Point3d {
    fn One() -> Point3d {
        return Point3d::new(1.0f64, 1.0f64, 1.0f64);
    }

    fn Zero() -> Point3d {
        return Point3d::new(0f64, 0f64, 0f64);
    }
}

impl UsualDatas<Point3f> for Point3f {
    fn One() -> Point3f {
        return Point3f::new(1.0f32, 1.0f32, 1.0f32);
    }

    fn Zero() -> Point3f {
        return Point3f::new(0f32, 0f32, 0f32);
    }
}
