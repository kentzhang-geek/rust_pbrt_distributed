use std::any::Any;
use fbxcel;
use fbxcel::low::v7400::AttributeType;
use fbxcel::pull_parser::any::{AnyParser, from_seekable_reader};
use fbxcel::tree::any::AnyTree;
use fbxcel::tree::v7400::NodeHandle;
use crate::core::tools::PrintSelf;
use crate::scene_file::scene_generated::{Scene, SceneArgs};

pub struct Reader {
}

fn foreach_child<'n>( n :NodeHandle<'n>) {
    n.name().show_self();
    for child in n.children() {
        foreach_child(child);
    }
}

impl crate::interface::io::Reader for Reader{
    fn fileToScene(filename: String) -> crate::scene_file::scene_generated::SceneArgs {
        let mut SceneArgs = SceneArgs::default();
        let file = std::fs::File::open(filename).expect("Failed to open file");
        let reader = std::io::BufReader::new(file);
        match AnyTree::from_seekable_reader(reader).expect("Failed to setup FBX parser") {
            // Use v7400 parser (implemented in `v7400` module).
            AnyTree::V7400(version, tree, footer) => {
                // You got a parser! Do what you want!
                tree.root().name().show_self();
                foreach_child(tree.root());
                // tree.show_self();
            },
            // `AnyParser` is nonexhaustive.
            // You should handle new unknown parser version case.
            _ => panic!("Unsupported FBX parser is required"),
        }
        return SceneArgs;
    }
}