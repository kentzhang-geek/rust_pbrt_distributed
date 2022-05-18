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
        let o = Vector3::new(1f64,2f64,3f64);
        let cs = rpd::core::coordinate_system::CoordinateSystem::New(
            o,o.clone_owned(),o.clone_owned()
        );
        cs.show_self();
    }
}
