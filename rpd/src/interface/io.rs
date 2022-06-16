pub trait Reader {
    fn fileToScene(filename : String)->crate::scene_file::scene_generated::SceneArgs;
}