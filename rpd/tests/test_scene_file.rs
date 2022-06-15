mod visualize;

#[cfg(test)]
mod tests_film {
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
    use rpd::scene_file::BVHAccel_generated::{BVHAccel, BVHAccelArgs};
    use rpd::scene_file::common_generated::Vec3d;

    #[test]
    fn test_scene_file_01() {
        let mut builder = FlatBufferBuilder::new();
        let mut args = BVHAccelArgs::default();
        let mut vs : Vec<Vec3d> = Vec::new();
        vs.push(Vec3d::new(1f64, 2f64, 3f64));
        vs.push(Vec3d::new(1f64, 2f64, 3f64));
        vs.push(Vec3d::new(1f64, 2f64, 3f64));
        args.test = Some(builder.create_vector(&vs));
        let mut v = BVHAccel::create(&mut builder, &BVHAccelArgs{
            test: None
        });
        builder.finish(v, None);
        builder.finished_data().len().show_self();
        println!("============flatbuffers=======");
    }
}
