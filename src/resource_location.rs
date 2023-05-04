use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use crate::error::ParseError;

/// A unique identifier for resources, consisting of a namespace and a path.
///
/// # Examples
///
/// ```
/// use ritematica::ResourceLocation;
///
/// let resource_location = ResourceLocation::new("create", "mechanical_drill");
/// assert_eq!(resource_location.get_namespace(), "create");
/// assert_eq!(resource_location.get_path(), "mechanical_drill");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ResourceLocation {
    pub(crate) namespace: String,
    pub(crate) path: String,
}

impl ResourceLocation {
    /// Returns the namespace of the `ResourceLocation` as a string slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use ritematica::ResourceLocation;
    ///
    /// let resource_location = ResourceLocation::new("create", "mechanical_drill");
    /// assert_eq!(resource_location.get_namespace(), "create");
    /// ```
    pub fn get_namespace(&self) -> &str {
        &self.namespace
    }

    /// Returns the path of the `ResourceLocation` as a string slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use ritematica::ResourceLocation;
    ///
    /// let resource_location = ResourceLocation::new("minecraft", "stone");
    /// assert_eq!(resource_location.get_path(), "stone");
    /// ```
    pub fn get_path(&self) -> &str {
        &self.path
    }

    /// Creates a new `ResourceLocation` with the given namespace and path.
    ///
    /// # Arguments
    ///
    /// * `namespace` - The namespace for the `ResourceLocation`. Must contain only ASCII alphanumeric characters, '_', '-', or '.'.
    /// * `path` - The path for the `ResourceLocation`. Must contain only ASCII alphanumeric characters, '_', '-', '/', or '.'.
    ///
    /// # Panics
    ///
    /// Panics if the namespace or path contains invalid characters.
    ///
    /// # Examples
    ///
    /// ```
    /// use ritematica::ResourceLocation;
    ///
    /// let resource_location = ResourceLocation::new("create", "mechanical_drill");
    /// ```
    pub fn new(namespace: impl Into<String>, path: impl Into<String>) -> Self {
        let namespace = namespace.into();
        let path = path.into();

        assert!(
            Self::is_valid_namespace(&namespace),
            "Invalid namespace {}",
            namespace
        );
        assert!(Self::is_valid_path(&path), "Invalid path {}", path);

        Self { namespace, path }
    }

    /// Creates a new `ResourceLocation` with the "minecraft" namespace and the given path.
    ///
    /// # Arguments
    ///
    /// * `path` - The path for the `ResourceLocation`. Must contain only ASCII alphanumeric characters, '_', '-', '/', or '.'.
    ///
    /// # Examples
    ///
    /// ```
    /// use my_crate::ResourceLocation;
    ///
    /// let resource_location = ResourceLocation::minecraft("andesite");
    /// assert_eq!(resource_location.get_namespace(), "minecraft");
    /// assert_eq!(resource_location.get_path(), "andesite");
    /// ```
    pub fn minecraft(path: impl Into<String>) -> Self {
        Self::new("minecraft", path)
    }

    /// Parses a string representation of a `ResourceLocation` into a `ResourceLocation` instance.
    ///
    /// The input string should be in the format "namespace:path". If the namespace is omitted,
    /// the default "minecraft" namespace will be used.
    ///
    /// # Arguments
    ///
    /// * `resource` - A string representation of a `ResourceLocation` in the format "namespace:path" or "path".
    ///
    /// # Errors
    ///
    /// Returns a `ParseError` if the input string is not a valid `ResourceLocation`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ritematica::{ResourceLocation, ParseError};
    ///
    /// let parsed_resource = ResourceLocation::parse("custom_namespace:resource_path").unwrap();
    /// assert_eq!(parsed_resource.get_namespace(), "custom_namespace");
    /// assert_eq!(parsed_resource.get_path(), "resource_path");
    ///
    /// let parsed_resource_default = ResourceLocation::parse("stone").unwrap();
    /// assert_eq!(parsed_resource_default.get_namespace(), "minecraft");
    /// assert_eq!(parsed_resource_default.get_path(), "stone");
    ///
    /// assert!(ResourceLocation::parse("invalid@namespace:stone").is_err());
    /// ```
    pub fn parse(resource: impl AsRef<str>) -> Result<Self, ParseError> {
        let resource = resource.as_ref();
        let mut split = resource.splitn(2, ':');

        let first = split.next().ok_or(ParseError)?;

        if let Some(second) = split.next() {
            if !Self::is_valid_namespace(first) {
                return Err(ParseError);
            }

            if !Self::is_valid_path(second) {
                return Err(ParseError);
            }

            Ok(Self {
                namespace: first.to_string(),
                path: second.to_string(),
            })
        } else {
            if !Self::is_valid_path(first) {
                return Err(ParseError);
            }

            Ok(Self {
                namespace: "minecraft".to_string(),
                path: first.to_string(),
            })
        }
    }

    fn is_valid_namespace(namespace: &str) -> bool {
        if namespace.is_empty() {
            return false;
        }

        namespace
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-' || c == '.')
    }

    fn is_valid_path(path: &str) -> bool {
        if path.is_empty() {
            return false;
        }

        path.chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-' || c == '/' || c == '.')
    }
}

impl<T> From<T> for ResourceLocation
where
    T: AsRef<str>,
{
    fn from(s: T) -> Self {
        Self::parse(s).expect("Failed to parse ResourceLocation")
    }
}

impl Display for ResourceLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.namespace, self.path)
    }
}

impl FromStr for ResourceLocation {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl Serialize for ResourceLocation {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&format!("{}", self))
    }
}

impl<'de> Deserialize<'de> for ResourceLocation {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;

        Self::parse(s).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_resource_location() {
        let resource_location = ResourceLocation::new("create", "mechanical_drill");

        assert_eq!(resource_location.get_namespace(), "create");
        assert_eq!(resource_location.get_path(), "mechanical_drill");
    }

    #[test]
    fn minecraft_resource_location() {
        let resource_location = ResourceLocation::minecraft("andesite");

        assert_eq!(resource_location.get_namespace(), "minecraft");
        assert_eq!(resource_location.get_path(), "andesite");
    }

    #[test]
    fn parse_resource_location() {
        let resource_location = ResourceLocation::parse("create:mechanical_drill").unwrap();

        assert_eq!(resource_location.get_namespace(), "create");
        assert_eq!(resource_location.get_path(), "mechanical_drill");
    }

    #[test]
    fn parse_resource_location_default_namespace() {
        let resource_location = ResourceLocation::parse("andesite").unwrap();

        assert_eq!(resource_location.get_namespace(), "minecraft");
        assert_eq!(resource_location.get_path(), "andesite");
    }

    #[test]
    fn parse_resource_location_invalid_namespace() {
        let result = ResourceLocation::parse("invalid!namespace:resource_path");

        assert!(result.is_err());
    }

    #[test]
    fn parse_resource_location_invalid_path() {
        let result = ResourceLocation::parse("custom_namespace:invalid!path");

        assert!(result.is_err());
    }

    #[test]
    fn resource_location_display() {
        let resource_location = ResourceLocation::new("create", "mechanical_drill");
        let display = format!("{}", resource_location);

        assert_eq!(display, "create:mechanical_drill");
    }

    #[test]
    fn resource_location_from_str() {
        let resource_location: ResourceLocation = "create:mechanical_bearing".parse().unwrap();

        assert_eq!(resource_location.get_namespace(), "create");
        assert_eq!(resource_location.get_path(), "mechanical_bearing");
    }
}
