use nalgebra;
use nalgebra::{dvector, OPoint};
use nalgebra::vector;

pub type Vector2d = nalgebra::Vector2<f64>;
pub type Vector2f = nalgebra::Vector2<f32>;

pub type Vector3d = nalgebra::Vector3<f64>;
pub type Vector3f = nalgebra::Vector3<f32>;

pub type Point2d = nalgebra::Point2<f64>;
pub type Point2f = nalgebra::Point2<f32>;

pub type Point3d = nalgebra::Point3<f64>;
pub type Point3f = nalgebra::Point3<f32>;

pub type Vector3 = Vector3d;
pub type Point3 = Point3d;

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

impl UsualDatas<Vector2d> for Vector2d {
    fn One() -> Vector2d {
        return Vector2d::new(1f64, 1f64);
    }
    fn Zero() -> Vector2d {
        return Vector2d::new(0f64, 0f64);
    }
}

impl UsualDatas<Vector2f> for Vector2f {
    fn One() -> Vector2f { return Vector2f::new(1f32, 1f32); }

    fn Zero() -> Vector2f { return Vector2f::new(0f32, 0f32); }
}

impl UsualDatas<Point2d> for Point2d {
    fn One() -> Point2d {
        return Point2d::new(1.0f64, 1.0f64);
    }

    fn Zero() -> Point2d {
        return Point2d::new(0f64, 0f64);
    }
}

impl UsualDatas<Point2f> for Point2f {
    fn One() -> Point2f {
        return Point2f::new(1.0f32, 1.0f32);
    }

    fn Zero() -> Point2f {
        return Point2f::new(0f32, 0f32);
    }
}
