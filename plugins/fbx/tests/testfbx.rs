pub mod tools;

use tools::*;

#[cfg(test)]
mod tests {
    use fbx::fileToScene;
    use scene_file::*;
    use tools::PrintSelf;
    use crate::tools;

    #[test]
    fn it_works() {
        println!("{:?}", std::env::current_dir());
        let ret = fileToScene(String::from("samplefbx/source/scene.fbx"));
        if let Some(data) = ret {
            let d = data.as_slice().clone();
            let s = scene::get_root_as_scene(d);
            s.root().unwrap().children().unwrap().get(0).children().unwrap().get(0).children().unwrap().len().show_self();
        }
    }
}
