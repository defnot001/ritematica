use crate::BlockState;
use std::collections::HashMap;

#[derive(Debug)]
pub struct BlockStateBuilder {
    name: String,
    properties: HashMap<String, String>,
}

impl BlockStateBuilder {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            properties: HashMap::new(),
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        let name = name.into();

        if !name.starts_with("minecraft:") {
            self.name = format!("minecraft:{}", name);
            return self;
        }

        self.name = name;
        self
    }

    pub fn properties(
        mut self,
        properties: impl IntoIterator<Item = (impl Into<String>, impl Into<String>)>,
    ) -> Self {
        for (key, value) in properties {
            self.properties.insert(key.into(), value.into());
        }

        self
    }

    pub fn build(self) -> BlockState {
        BlockState {
            name: self.name,
            properties: self.properties,
        }
    }
}

impl BlockState {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_properties(&self) -> &HashMap<String, String> {
        &self.properties
    }

    /// Sets the name of the block state. If the name does not start with `minecraft:`, it will be added to the beginning.
    ///
    /// # Examples
    ///
    /// ```
    /// let block_state = ...; // Load or create a BlockState
    /// block_state.set_name("stone");
    /// ```
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the block state
    pub fn set_name(&mut self, name: impl Into<String>) {
        let name: String = name.into();

        // check if the name starts with minecraft:, otherwise add it to the beginning
        if !name.starts_with("minecraft:") {
            self.name = format!("minecraft:{}", name);
            return;
        }

        self.name = name;
    }

    /// Sets the properties of the block state. This will overwrite any existing properties.
    /// If you want to add properties, use [`add_properties`](#method.add_properties) instead.
    ///
    /// # Examples
    ///
    /// ```
    /// let block_state = ...; // Load or create a BlockState
    /// block_state.set_properties([("facing", "down"), ("extended", "true")]);
    /// ```
    ///
    /// # Arguments
    ///
    /// * `properties` - The properties of the block state
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

    /// Adds the given properties to the block state.
    ///
    /// # Examples
    ///
    /// ```
    /// let block_state = ...; // Load or create a BlockState
    /// block_state.add_properties([("facing", "down"), ("extended", "true")]);
    /// ```
    ///
    /// # Arguments
    ///
    /// * `properties` - The properties of the block state
    pub fn add_properties<K, V>(&mut self, properties: impl IntoIterator<Item = (K, V)>)
    where
        K: Into<String>,
        V: Into<String>,
    {
        for (key, value) in properties {
            self.properties.insert(key.into(), value.into());
        }
    }

    /// Removes all properties from the block state.
    pub fn clear_properties(&mut self) {
        self.properties.clear();
    }

    /// Removes the given property from the block state.
    /// If the property does not exist, nothing will happen.
    /// If you want to remove all properties, use [`clear_properties`](#method.clear_properties) instead.
    pub fn remove_property(&mut self, property: impl Into<String>) {
        self.properties.remove(&property.into());
    }
}
