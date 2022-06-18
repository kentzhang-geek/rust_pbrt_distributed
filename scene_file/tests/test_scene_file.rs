pub mod tools;

#[cfg(test)]
mod tests_film {
    use flatbuffers::FlatBufferBuilder;
    use scene_file::bvh_accel_generated::{BVHAccel, BVHAccelArgs};
    use scene_file::common_generated::Vec3d;
    use super::tools::*;

    #[test]
    fn test_scene_file_01() {
        let mut builder = FlatBufferBuilder::new();
        let mut args = BVHAccelArgs::default();
        let mut vs : Vec<Vec3d> = Vec::new();
        vs.push(Vec3d::new(1f64, 2f64, 3f64));
        vs.push(Vec3d::new(1f64, 2f64, 3f64));
        vs.push(Vec3d::new(1f64, 2f64, 3f64));
        let mut v = BVHAccel::create(&mut builder, &args);
        builder.finish(v, None);
        builder.finished_data().len().show_self();
        let data = builder.finished_data();
        println!("============flatbuffers=======");

        let bvh = flatbuffers::root::<BVHAccel>(data).unwrap();
        bvh.show_self();
    }
}
