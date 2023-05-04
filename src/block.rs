use crate::{resource_location::ResourceLocation, structure::BlockState};
use std::collections::HashMap;

/// A pattern that can be used to match block states.
pub trait BlockStatePattern {
    /// Checks whether the given `block_state` matches the pattern.
    ///
    /// # Arguments
    ///
    /// * `block_state` - The `BlockState` to be tested for a match.
    ///
    /// # Returns
    ///
    /// * `bool` - Returns `true` if the `block_state` matches the pattern, otherwise returns `false`.
    fn matches(&self, block_state: &BlockState) -> bool;
}

/// A builder for creating `BlockState`s.
///
/// # Examples
/// ```
/// use ritematica::BlockStateBuilder;
///
/// let blockstate = BlockStateBuilder::new("piston")
///     .properties([("facing", "down")])
///     .build();
/// ```
#[derive(Debug)]
pub struct BlockStateBuilder {
    name: ResourceLocation,
    properties: HashMap<String, String>,
}

impl BlockStateBuilder {
    /// Creates a new `BlockStateBuilder` for a block with a given name.
    /// After creating the builder, you can either add properties using the `properties()` method or directly build the `BlockState` by calling the `build()` method.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the block as a `ResourceLocation` or a `String` in the format `namespace:name`. If no namespace is provided, `minecraft` is assumed.
    ///
    /// # Examples
    /// ```
    /// use ritematica::BlockStateBuilder;
    ///
    /// let block_state = BlockStateBuilder::new("piston").build();
    ///
    /// assert_eq!(block_state.get_name().to_string(), "minecraft:piston");
    /// ```
    pub fn new(name: impl Into<ResourceLocation>) -> Self {
        Self {
            name: name.into(),
            properties: HashMap::new(),
        }
    }

    /// Adds `properties` to the `BlockStateBuilder`.
    /// If a property with the same name already exists, it will be overwritten.
    ///
    /// # Arguments
    ///
    /// * `properties` - An iterator over tuples of the form `(name, value)`. The name and value must both be convertible to `String`.
    ///
    /// # Examples
    /// ```
    /// use ritematica::BlockStateBuilder;
    ///
    /// let block_state = BlockStateBuilder::new("piston")
    ///    .properties([("facing", "down")])
    ///    .build();
    /// ```
    pub fn properties(
        mut self,
        properties: impl IntoIterator<Item = (impl Into<String>, impl Into<String>)>,
    ) -> Self {
        for (key, value) in properties {
            self.properties.insert(key.into(), value.into());
        }

        self
    }

    /// Builds the `BlockState` from the `BlockStateBuilder`.
    ///
    /// # Examples
    /// ```
    /// use ritematica::BlockStateBuilder;
    ///
    /// let block_state = BlockStateBuilder::new("piston")
    ///    .properties([("facing", "down")])
    ///    .build();
    /// ```
    pub fn build(self) -> BlockState {
        BlockState {
            name: self.name,
            properties: self.properties,
        }
    }
}

impl BlockState {
    /// Returns the name of a `BlockState` as a reference to a `ResourceLocation`.
    ///
    /// # Examples
    /// ```
    /// use ritematica::BlockStateBuilder;
    ///
    /// let blockstate = BlockStateBuilder::new("piston")
    ///     .properties([("facing", "down")])
    ///     .build();
    ///
    /// assert_eq!(
    ///     blockstate.get_name(),
    ///     &ResourceLocation {
    ///         namespace: "minecraft".to_string(),
    ///         path: "piston".to_string(),
    ///     }
    /// );
    /// ```
    pub fn get_name(&self) -> &ResourceLocation {
        &self.name
    }

    /// Returns the properties of a `BlockState` as a reference to a `HashMap<String, String>`.
    ///
    /// # Examples
    /// ```
    /// use ritematica::BlockStateBuilder;
    ///
    /// let blockstate = BlockStateBuilder::new("piston")
    ///    .properties([("facing", "down")])
    ///    .build();
    ///
    /// assert_eq!(
    ///     blockstate.get_properties(),
    ///     &[("facing".to_string(), "down".to_string())]
    ///         .iter()
    ///         .cloned()
    ///         .collect::<HashMap<String, String>>()
    /// );
    /// ```
    pub fn get_properties(&self) -> &HashMap<String, String> {
        &self.properties
    }

    /// Sets the name of a `BlockState`.
    ///
    /// # Arguments
    ///
    /// * `name` - The new name of the block as a `ResourceLocation` or a `String` in the format `namespace:name`. If no namespace is provided, `minecraft` is assumed.
    ///
    /// # Examples
    /// ```
    /// use ritematica::{BlockStateBuilder, BlockState};
    ///
    /// let mut blockstate = BlockStateBuilder::new("piston")
    ///     .properties([("facing", "down")])
    ///     .build();
    ///
    /// blockstate.set_name("sticky_piston");
    ///
    /// assert_eq!(
    ///     blockstate.get_name(),
    ///     &ResourceLocation {
    ///         namespace: "minecraft".to_string(),
    ///         path: "sticky_piston".to_string(),
    ///     }
    /// );
    /// ```
    pub fn set_name(&mut self, name: impl Into<ResourceLocation>) {
        self.name = name.into();
    }

    /// Sets the properties of a `BlockState`. Clears any existing properties before adding the new ones.
    ///
    /// # Arguments
    ///
    /// * `properties` - An iterator over tuples of the form `(name, value)`. The name and value must both be convertible to `String`.
    ///
    /// # Examples
    /// ```
    /// use ritematica::{BlockStateBuilder, BlockState};
    ///
    /// let mut blockstate = BlockStateBuilder::new("piston")
    ///     .properties([("facing", "down")])
    ///     .build();
    ///
    /// blockstate.set_properties([("facing", "up")]);
    ///
    /// assert_eq!(
    ///     blockstate.get_properties(),
    ///     &[("facing".to_string(), "up".to_string())]
    ///         .iter()
    ///         .cloned()
    ///         .collect::<HashMap<String, String>>()
    /// );
    /// ```
    pub fn set_properties<K, V>(&mut self, properties: impl IntoIterator<Item = (K, V)>)
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.properties.clear();

        for (key, value) in properties {
            self.properties.insert(key.into(), value.into());
        }
    }

    /// Adds properties to a `BlockState`. If a property with the same name already exists, it will be overwritten.
    ///
    /// # Arguments
    ///
    /// * `properties` - An iterator over tuples of the form `(name, value)`. The name and value must both be convertible to `String`.
    ///
    /// # Examples
    /// ```
    /// use ritematica::{BlockStateBuilder, BlockState};
    ///
    /// let mut blockstate = BlockStateBuilder::new("piston")
    ///     .properties([("facing", "down")])
    ///     .build();
    ///
    /// blockstate.add_properties([("extended", "true")]);
    ///
    /// assert_eq!(
    ///     blockstate.get_properties(),
    ///     &[("facing".to_string(), "down".to_string()), ("extended".to_string(), "true".to_string())]
    ///         .iter()
    ///         .cloned()
    ///         .collect::<HashMap<String, String>>()
    /// );
    /// ```
    pub fn add_properties<K, V>(&mut self, properties: impl IntoIterator<Item = (K, V)>)
    where
        K: Into<String>,
        V: Into<String>,
    {
        for (key, value) in properties {
            self.properties.insert(key.into(), value.into());
        }
    }

    /// Removes all properties from a `BlockState`.
    ///
    /// # Examples
    /// ```
    /// use ritematica::{BlockStateBuilder, BlockState};
    ///
    /// let mut blockstate = BlockStateBuilder::new("piston")
    ///     .properties([("facing", "down")])
    ///     .build();
    ///
    /// blockstate.clear_properties();
    ///
    /// assert_eq!(blockstate.get_properties().len(), 0);
    /// ```
    pub fn clear_properties(&mut self) {
        self.properties.clear();
    }

    /// Removes a property from a `BlockState` by name.
    ///
    /// # Arguments
    ///
    /// * `property` - The name of the property to be removed.
    ///
    /// # Examples
    /// ```
    /// use ritematica::{BlockStateBuilder, BlockState};
    ///
    /// let mut blockstate = BlockStateBuilder::new("piston")
    ///     .properties([("facing", "down")])
    ///     .build();
    ///
    /// blockstate.remove_property("facing");
    ///
    /// assert_eq!(blockstate.get_properties().len(), 0);
    /// ```
    pub fn remove_property(&mut self, property: impl Into<String>) {
        self.properties.remove(&property.into());
    }
}

impl BlockStatePattern for BlockState {
    fn matches(&self, block_state: &BlockState) -> bool {
        self == block_state
    }
}

impl<T> BlockStatePattern for T
where
    T: Fn(&BlockState) -> bool,
{
    /// Checks whether the given `block_state` matches the pattern implemented by the closure.
    ///
    /// # Arguments
    ///
    /// * `block_state` - The `BlockState` to be tested for a match.
    ///
    /// # Returns
    ///
    /// * `bool` - Returns `true` if the `block_state` matches the pattern implemented by the closure, otherwise returns `false`.
    ///
    /// # Example
    ///
    /// ```
    /// use ritematica::{BlockStateBuilder, BlockStatePattern};
    ///
    /// let block_state = BlockStateBuilder::new("minecraft:piston")
    ///     .properties([("facing", "down")])
    ///     .build();
    ///
    /// let is_piston_facing_down = |block_state: &BlockState| {
    ///     block_state.get_name().path == "piston" && block_state.get_properties().get("facing") == Some(&"down".to_string())
    /// };
    ///
    /// assert_eq!(is_piston_facing_down.matches(&block_state), true);
    /// ```
    fn matches(&self, block_state: &BlockState) -> bool {
        self(block_state)
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn builder() {
        let blockstate = BlockStateBuilder::new("piston")
            .properties([("facing", "down")])
            .build();

        assert_eq!(
            blockstate.name,
            ResourceLocation {
                namespace: "minecraft".to_string(),
                path: "piston".to_string(),
            }
        );

        assert_eq!(
            blockstate.properties,
            [("facing".to_string(), "down".to_string())]
                .iter()
                .cloned()
                .collect::<HashMap<String, String>>()
        );
    }

    #[test]
    fn blockstate() {
        let blockstate = BlockState {
            name: ResourceLocation {
                namespace: "minecraft".to_string(),
                path: "piston".to_string(),
            },
            properties: [
                ("facing".to_string(), "down".to_string()),
                ("extended".to_string(), "true".to_string()),
            ]
            .iter()
            .cloned()
            .collect::<HashMap<String, String>>(),
        };

        // testing get_name()
        let get_name = blockstate.get_name();

        assert_eq!(
            get_name,
            &ResourceLocation {
                namespace: "minecraft".to_string(),
                path: "piston".to_string(),
            }
        );

        // testing get_properties()
        let get_properties = blockstate.get_properties();

        assert_eq!(
            get_properties,
            &[
                ("facing".to_string(), "down".to_string()),
                ("extended".to_string(), "true".to_string())
            ]
            .iter()
            .cloned()
            .collect::<HashMap<String, String>>()
        );

        // testing set_name()
        let mut blockstate = blockstate.clone();
        blockstate.set_name("sticky_piston");

        assert_eq!(
            blockstate.name,
            ResourceLocation {
                namespace: "minecraft".to_string(),
                path: "sticky_piston".to_string(),
            }
        );

        // testing set_properties()
        blockstate.set_properties([("facing", "up")]);

        assert_eq!(
            blockstate.properties,
            [("facing".to_string(), "up".to_string())]
                .iter()
                .cloned()
                .collect::<HashMap<String, String>>()
        );
    }

    #[test]
    fn blockstate_add_properties() {
        let mut blockstate = BlockStateBuilder::new("piston")
            .properties([("facing", "down")])
            .build();

        blockstate.add_properties([("extended", "true")]);

        assert_eq!(
            blockstate.properties,
            [
                ("facing".to_string(), "down".to_string()),
                ("extended".to_string(), "true".to_string()),
            ]
            .iter()
            .cloned()
            .collect::<HashMap<String, String>>()
        );
    }

    #[test]
    fn blockstate_clear_properties() {
        let mut blockstate = BlockStateBuilder::new("piston")
            .properties([("facing", "down"), ("extended", "true")])
            .build();

        blockstate.clear_properties();

        assert_eq!(blockstate.properties, HashMap::<String, String>::new());
    }

    #[test]
    fn blockstate_remove_property() {
        let mut blockstate = BlockStateBuilder::new("piston")
            .properties([("facing", "down"), ("extended", "true")])
            .build();

        blockstate.remove_property("extended");

        assert_eq!(
            blockstate.properties,
            [("facing".to_string(), "down".to_string())]
                .iter()
                .cloned()
                .collect::<HashMap<String, String>>()
        );
    }

    #[test]
    fn blockstate_pattern_matches() {
        let pattern = BlockStateBuilder::new("piston")
            .properties([("facing", "down")])
            .build();

        let block_state = BlockStateBuilder::new("piston")
            .properties([("facing", "down")])
            .build();

        assert!(pattern.matches(&block_state));
    }

    #[test]
    fn blockstate_pattern_does_not_match() {
        let pattern = BlockStateBuilder::new("piston")
            .properties([("facing", "down")])
            .build();

        let block_state = BlockStateBuilder::new("piston")
            .properties([("facing", "up")])
            .build();

        assert!(!pattern.matches(&block_state));
    }

    #[test]
    fn blockstate_pattern_fn_matches() {
        let pattern_fn: Box<dyn BlockStatePattern> = Box::new(|block_state: &BlockState| {
            block_state.get_name().to_string() == "minecraft:piston"
                && block_state.get_properties().get("facing") == Some(&"down".to_string())
        });

        let block_state = BlockStateBuilder::new("piston")
            .properties([("facing", "down")])
            .build();

        assert!(pattern_fn.matches(&block_state));
    }
}
