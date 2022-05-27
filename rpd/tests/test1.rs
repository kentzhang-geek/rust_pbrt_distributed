#[cfg(test)]
mod tests {
    use rpd::interface::shape;
    use rpd::interface::shape::Shape;

    #[test]
    fn test01() {
        let s = shape::STest { v: 12i32 };
        s.test();
    }
}

#[cfg(test)]
mod tests_core {
    use rpd::core::math::*;
    use rpd::core::tools::*;

    #[test]
    fn test02() {
        let a = Vector3d::Zero();
        let o = Vector3d::new(1f64,2f64,3f64);
        let cs = rpd::core::coordinate_system::CoordinateSystem::New(
            o,o.clone_owned(),o.clone_owned()
        );
        cs.show_self();
    }
}

#[cfg(test)]
mod test_other {
    use rpd::core::geometry::Ray;
    use rpd::core::math::*;
    use rpd::core::tools::PrintSelf;

    #[test]
    fn test_ray_01() {
        let r = Ray::new(Point3d::Zero(), Vector3d::new(1.0f64, 0f64, 0f64));
        r.show_self();
        let res = r.at(12f64);
        assert!(res.x == 12.0f64);
    }
}
