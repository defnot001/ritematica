use std::borrow::Borrow;
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufReader, BufWriter};
use std::path::Path;

use crate::error::Result;
use crate::structure::{LitematicaFile, Region};

impl LitematicaFile {
    /// Reads a `Litematica` file from the given path.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the file.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be opened or deserialized.
    ///
    /// # Examples
    /// ```
    /// use ritematica::LitematicaFile;
    ///
    /// let file = LitematicaFile::read("test.litematic").unrwrap();
    ///```
    pub fn read(path: impl AsRef<Path>) -> Result<LitematicaFile> {
        let file = File::open(path)?;
        let buf_reader = BufReader::new(file);
        Ok(nbt::from_gzip_reader(buf_reader)?)
    }

    /// Writes a `Litematica` file to the given path.
    ///
    /// Depending on the platform, this function may fail if the full directory `path` does not exist.
    ///
    /// # Arguments
    ///
    /// * `path` - The path where the file should be written to.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be created or serialized.
    /// Also returns an error if the file extension is not `.litematic`.
    ///
    /// # Examples
    /// ```
    /// use ritematica::LitematicaFile;
    ///
    /// let file = LitematicaFile::read("test.litematic").unrwrap();
    /// file.write("test2.litematic").unrwrap();
    /// ```
    pub fn write(&self, path: impl AsRef<Path>) -> Result<()> {
        if let Some(ext) = path.as_ref().extension() {
            if ext != "litematic" {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "File extension must be .litematic",
                )
                .into());
            }
        }

        let file = File::create(path)?;
        let mut buf_writer = BufWriter::new(file);
        nbt::to_gzip_writer(&mut buf_writer, self, None)?;

        Ok(())
    }

    /// Returns a reference to a `HashMap` containing all the `regions` in the file.
    ///
    /// The `HashMap` is keyed by the region's `name`. The value is the region `data`.
    ///
    /// # Examples
    /// ```
    /// use ritematica::LitematicaFile;
    ///
    /// let file = LitematicaFile::read("test.litematic").unwrap();
    /// let regions = file.get_regions();
    ///
    /// assert_eq!(regions.len(), 1);
    /// assert!(regions.contains_key("test"));
    /// ```
    pub fn get_regions(&self) -> &HashMap<String, Region> {
        &self.regions
    }

    /// Returns a mutable reference to a `HashMap` containing all the `regions` in the file.
    ///
    /// The `HashMap` is keyed by the region's `name`. The value is the region `data`. Use this function only if you need to modify the regions and don't want to use the built-in functions.
    ///
    /// # Examples
    /// ```
    /// use ritematica::LitematicaFile;
    ///
    /// let mut file = LitematicaFile::read("test.litematic").unwrap();
    /// let regions = file.get_regions_mut();
    ///
    /// assert_eq!(regions.len(), 1);
    /// assert!(regions.contains_key("test"));
    /// ```
    pub fn get_regions_mut(&mut self) -> &mut HashMap<String, Region> {
        &mut self.regions
    }

    /// Returns an `iterator` over the region `names` in a `litematica` file.
    ///
    /// # Examples
    /// ```
    /// use ritematica::LitematicaFile;
    ///
    /// let file = LitematicaFile::read("test.litematic").unwrap();
    /// let region_names = file.get_region_names();
    ///
    /// assert_eq!(region_names.next(), Some("test"));
    /// ```
    pub fn get_region_names(&self) -> impl Iterator<Item = &str> {
        self.regions.keys().map(|s| s.as_str())
    }

    /// Returns an `Option` containing a reference to the `region` with the given name.
    /// If the region does not exist, `None` is returned.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the region.
    ///
    /// # Examples
    /// ```
    /// use ritematica::LitematicaFile;
    ///
    /// let file = LitematicaFile::read("test.litematic").unwrap();
    /// let region = file.get_region("test");
    ///
    /// assert!(region.is_some());
    /// ```
    pub fn get_region<Q: ?Sized>(&self, name: &Q) -> Option<&Region>
    where
        String: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.regions.get(name)
    }

    /// Returns an `Option` containing a mutable reference to the `region` with the given name.
    /// If the region does not exist, `None` is returned.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the region.
    ///
    /// # Examples
    /// ```
    /// use ritematica::LitematicaFile;
    ///
    /// let mut file = LitematicaFile::read("test.litematic").unwrap();
    /// let region = file.get_region_mut("test");
    ///
    /// assert!(region.is_some());
    /// ```
    pub fn get_region_mut<Q: ?Sized>(&mut self, name: &Q) -> Option<&mut Region>
    where
        String: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.regions.get_mut(name)
    }

    /// Renames a `region` with the given `old_name` to the given `new_name`.
    /// If the region does not exist, nothing happens.
    ///
    /// # Arguments
    ///
    /// * `old_name` - The name of the region to rename.
    /// * `new_name` - The new name of the region.
    ///
    /// # Examples
    /// ```
    /// use ritematica::LitematicaFile;
    ///
    /// let mut file = LitematicaFile::read("test.litematic").unwrap();
    /// file.rename_region("test", "test2");
    ///
    /// assert!(file.get_region("test").is_none());
    /// assert!(file.get_region("test2").is_some());
    /// ```
    pub fn rename_region<Q: ?Sized>(&mut self, old_name: &Q, new_name: impl Into<String>)
    where
        String: Borrow<Q>,
        Q: Hash + Eq,
    {
        let removed = self.regions.remove(old_name);

        if let Some(region) = removed {
            self.regions.insert(new_name.into(), region);
        }
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn get_regions() {
        let file = LitematicaFile::read("test.litematic").unwrap();
        let regions = file.get_regions();

        assert_eq!(regions.len(), 1);
        assert!(regions.contains_key("test"));
    }

    #[test]
    fn get_region_names() {
        let file = LitematicaFile::read("test.litematic").unwrap();
        let mut region_names = file.get_region_names();

        assert_eq!(region_names.next(), Some("test"));
    }

    #[test]
    fn get_region() {
        let file = LitematicaFile::read("test.litematic").unwrap();
        let region = file.get_region("test");

        assert!(region.is_some());
    }

    #[test]
    fn rename_region() {
        let mut file = LitematicaFile::read("test.litematic").unwrap();
        file.rename_region("test", "test2");

        assert!(file.get_region("test").is_none());
        assert!(file.get_region("test2").is_some());
    }
}
