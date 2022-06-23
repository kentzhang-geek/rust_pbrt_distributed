mod visualize;

#[cfg(test)]
mod tests_film {
    use std::collections::LinkedList;
    use std::fmt::Debug;
    use std::ops::Mul;
    use std::sync::{Arc, Weak};
    use gltf::Gltf;
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
    use rpd::interface::io::Pack;
    use rpd::shapes::mesh::GenerateMesh;
    use scene_file::bvh_accel::{BVHNode, BVHNodeT};
    use fbx::*;

    #[test]
    fn t01() {
        let mut s = Sphere::newOnlyMove(Vector3d::new(1f64, 1f64, 1f64), 2f64);
        let mut meshp = ShapePrimitive {
            shape: Arc::new(s.toMesh(12i32)),
            material: None,
            areaLight: None,
        };
        let mut shapelist: LinkedList<Arc<dyn Primitive>> = LinkedList::new();
        shapelist.push_back(Arc::new(meshp));
        let mut bvh = BVHAccel::make(&shapelist, 2i32).unwrap();
        // save to file
        let st = bvh.pack();
        let mut builder = flatbuffers::FlatBufferBuilder::new();
        let mut stpacked = st.pack(&mut builder);
        builder.finish(stpacked, None);
        let mut data = Vec::from(builder.finished_data());

        fbx::sceneToFile(String::from("testout.fbx"), data).show_self();

        println!("Tested");
    }
}