mod error;
#[allow(unused)]
mod parsing;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::path::Path;

    use crate::parsing::{BlockState, LitematicaFile};

    #[test]
    fn it_works() {
        let mut litematica_file = LitematicaFile::read(Path::new("test.litematic")).unwrap();

        let region = litematica_file
            .regions
            .get_mut("Yisibite_4module_tripleshot_tunnel_bore")
            .unwrap();

        println!("{:?}", region.get_block((5, 3, 5)));

        let mut map = HashMap::new();

        map.insert("facing".to_string(), "down".to_string());
        map.insert("extended".to_string(), "true".to_string());

        region.set_block(
            (5, 3, 5),
            BlockState {
                name: "minecraft:piston".to_string(),
                properties: map,
            },
        );

        println!("{:?}", region.get_block((5, 3, 5)));

        litematica_file
            .write(Path::new("test_new.litematic"))
            .unwrap();
    }
}
