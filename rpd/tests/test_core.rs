#[cfg(test)]
mod tests_film {
    use rpd::interface::shape;
    use rpd::core::film;
    use rpd::core::math::*;
    use nalgebra_glm::*;
    use rpd::core::film::Pixel;
    use rpd::core::geometry::{Bounds3, Ray};
    use rpd::core::tools::PrintSelf;
    use rpd::core::transform::Transform;
    use rpd::interface::shape::Shape;
    use rpd::shapes::Sphere;

    #[test]
    fn test_bounds3() {
        let mut bounds3 = Bounds3{ pMin: Point3d::new(-1f64, -2f64, -1f64), pMax: Point3d::new(3f64, 2f64, 1f64)};
        let mut r = Ray::new(Point3d::new(-2f64, 0f64, 0f64), Vector3d::new(1.0f64, 0f64, 0f64));
        let result = bounds3.IntersectP(&r);
        result.show_self();
        if let Ok(res) = result {
            r.at(res.0).show_self();
            r.at(res.1).show_self();
        } else {
            assert!(false);
        }
        let mut r = Ray::new(Point3d::new(-2f64, 0f64, 0f64), Vector3d::new(1.0f64, 1f64, -1f64));
        let result = bounds3.IntersectP(&r);
        result.show_self();
        if let Ok(res) = result {
            r.at(res.0).show_self();
            r.at(res.1).show_self();
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_sphere() {
        let mut trans = Transform::new(&Matrix44d::new_translation(&Vector3d::new(1f64, 2f64, 1f64)));
        let mut sph = Sphere::new(&trans, 12.0f64);
        sph.show_self();
        sph.worldBound().show_self();
    }
}
