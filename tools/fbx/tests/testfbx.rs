pub mod tools;

use tools::*;

#[cfg(test)]
mod tests {
    use fbx::{fileToScene, sceneToFile};
    use flatbuffers;
    use scene_file::*;
    use scene_file::scene::Scene;
    use tools::PrintSelf;
    use crate::tools;

    #[test]
    fn it_works() {
        println!("{:?}", std::env::current_dir());
        let ret = fileToScene(String::from("samplefbx/source/scene.fbx"));
        if let Some(data) = ret {
            let d = data.as_slice().clone();
            let s = flatbuffers::root::<Scene>(d);
            if let Ok(s) = s {
                s.root().unwrap().name().show_self();
            }
            // sceneToFile(String::from("testout.fbx"), data.clone());
        }
    }
}
