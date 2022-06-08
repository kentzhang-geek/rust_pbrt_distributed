mod visualize;

#[cfg(test)]
mod tests_film {
    use std::collections::LinkedList;
    use std::fmt::Debug;
    use std::ops::Mul;
    use std::sync::{Arc, Weak};
    use image::codecs::png::CompressionType::Default;
    use nalgebra::sup;
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
    use three;
    use three::Object;
    use rpd::accelerators::bvh::BVHAccel;
    use rpd::core::interaction::{InteractionBase, SurfaceInteraction, SurfaceShading};
    use rpd::core::primitive::{Primitive, ShapePrimitive};

    #[test]
    fn test_bvh_01() {
        let mut test_data: Vec<(Vector3d, f64)> = vec![
            (Vector3d::new(0.0f64, 0f64, 0f64), 1f64),
            (Vector3d::new(0.0f64, 0f64, 2f64), 1f64),
            (Vector3d::new(0.0f64, 0f64, 4f64), 1f64),
            (Vector3d::new(0.0f64, 0f64, 6f64), 1f64),
        ];

        let mut spheres: LinkedList<Arc<dyn Primitive>> = LinkedList::new();
        for t in test_data {
            spheres.push_back(Arc::new(ShapePrimitive {
                shape: Arc::new((Sphere::newOnlyMove(t.0, t.1))),
                material: None,
                areaLight: None,
            }));
        }
        let mut node_root = BVHAccel::make(&spheres, 1);
        node_root.show_self();

        println!("Tested");
    }

    #[test]
    fn test_bvh_02() {
        let mut test_data: Vec<(Vector3d, f64)> = vec![
            // (Vector3d::new(0.0f64, 0f64, 0f64), 1f64),
            (Vector3d::new(0.0f64, 0f64, 2f64), 1f64),
            (Vector3d::new(0.0f64, 0f64, 4f64), 1f64),
            (Vector3d::new(0.0f64, 0f64, 6f64), 1f64),
            (Vector3d::new(3.0f64, 0f64, 2f64), 4f64),
        ];

        let mut spheres: LinkedList<Arc<dyn Primitive>> = LinkedList::new();
        for t in test_data {
            spheres.push_back(Arc::new(ShapePrimitive {
                shape: Arc::new((Sphere::newOnlyMove(t.0, t.1))),
                material: None,
                areaLight: None,
            }));
        }
        let mut node_root = BVHAccel::make(&spheres, 1).unwrap();
        node_root.show_self();

        let mut ray = Ray::new(vec3(0f64, 0f64, 0f64), vec3(0f64, 0f64, 1f64).normalize());
        let mut isect = SurfaceInteraction::default();
        node_root.intersect(& mut ray, & mut isect);
        isect.show_self();
        ray.show_self();

        println!("Tested");
    }
}
