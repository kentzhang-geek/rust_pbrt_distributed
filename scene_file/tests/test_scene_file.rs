pub mod tools;

#[cfg(test)]
mod tests_film {
    use flatbuffers::FlatBufferBuilder;
    use scene_file::scene::sf::*;
    use super::tools::*;

    #[test]
    fn test_scene_file_01() {
        let mut builder = FlatBufferBuilder::new();
        let mut args = SceneArgs::default();
        let mut vs : Vec<Vec3d> = Vec::new();
        vs.push(Vec3d::new(1f64, 2f64, 3f64));
        vs.push(Vec3d::new(1f64, 2f64, 3f64));
        vs.push(Vec3d::new(1f64, 2f64, 3f64));
        let mut v = Scene::create(&mut builder, &args);
        builder.finish(v, None);
        builder.finished_data().len().show_self();
        let data = builder.finished_data();
        println!("============flatbuffers=======");

        let bvh = flatbuffers::root::<Scene>(data).unwrap();
        bvh.show_self();
    }
}
