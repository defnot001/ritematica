#[allow(unused)]
mod block;
mod error;
mod file;
mod region;
mod structure;

pub use error::Error;
pub use structure::LitematicaFile;
pub use structure::{BlockState, Coordinates, Region};

#[cfg(test)]
mod tests {
    // use std::collections::HashMap;
    // use std::path::Path;

    use crate::block::BlockStateBuilder;

    #[test]
    fn it_works() {
        // let mut litematica_file = LitematicaFile::read(Path::new("test.litematic")).unwrap();

        // let region = litematica_file
        //     .regions
        //     .get_mut("Yisibite_4module_tripleshot_tunnel_bore")
        //     .unwrap();

        // println!("{:?}", region.get_block((5, 3, 5)));

        // let mut map = HashMap::new();

        // map.insert("facing".to_string(), "down".to_string());
        // map.insert("extended".to_string(), "true".to_string());

        // region.set_block(
        //     (5, 3, 5),
        //     BlockState {
        //         name: "minecraft:piston".to_string(),
        //         properties: map,
        //     },
        // );

        // println!("{:?}", region.get_block((5, 3, 5)));

        // litematica_file
        //     .write(Path::new("test_new.litematic"))
        //     .unwrap();

        let mut block = BlockStateBuilder::new()
            .name("piston")
            .properties([("facing", "down"), ("extended", "true")])
            .build();

        let name = block.get_name();
        let props = block.get_properties();
        assert_eq!(name, "minecraft:piston");
        assert_eq!(props.get("facing"), Some(&"down".to_string()));

        block.set_name("sticky_piston");
        assert_eq!(block.get_name(), "minecraft:sticky_piston");

        block.set_properties([("facing", "up")]);
        assert_eq!(
            block.get_properties().get("facing"),
            Some(&"up".to_string())
        );

        println!("{:?}", block);
    }
}
