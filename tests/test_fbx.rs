mod visualize;

#[cfg(test)]
mod test_fbx {
    use std::collections::LinkedList;
    use std::fmt::Debug;
    use std::ops::Mul;
    use std::sync::{Arc, Weak};
    use flatbuffers::FlatBufferBuilder;
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
    use rpd::core::interaction::{InteractionBase, SurfaceInteraction, SurfaceShading};
    use rpd::core::primitive::{Primitive, ShapePrimitive};
    use rpd::interface::io::Reader;
    use rpd::scene_file::bvh_accel_generated::{BVHAccel, BVHAccelArgs};
    use rpd::scene_file::common_generated::Vec3d;

    #[test]
    fn t001() {
        let mut filename = String::from("tests/toy_box/source/scene.fbx");
        let mut sargs = rpd::plugins::fbx::reader::Reader::fileToScene(filename);
    }
}
