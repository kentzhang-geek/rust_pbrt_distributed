#[cfg(test)]
mod tests {
    use fbx::fileToScene;

    #[test]
    fn it_works() {
        println!("{:?}", std::env::current_dir());
        fileToScene(String::from("samplefbx/source/scene.fbx"));
    }
}
