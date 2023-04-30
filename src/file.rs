use std::borrow::Borrow;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufReader, BufWriter};
use std::path::Path;

use crate::error::Result;
use crate::structure::{LitematicaFile, Region};

impl LitematicaFile {
    /// Reads a Litematica file from the given path.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the file.
    ///
    /// # Examples
    ///
    /// ```
    /// let litematica = LitematicaFile::read("path/to/schematic.litematic").unwrap();
    /// ```
    ///
    /// # Returns
    ///
    /// * A Result containing the Litematica file if successful, or an error if the file could not be read and parsed.
    pub fn read(path: impl AsRef<Path>) -> Result<LitematicaFile> {
        let file = File::open(path)?;
        let buf_reader = BufReader::new(file);
        Ok(nbt::from_gzip_reader(buf_reader)?)
    }

    /// Writes the Litematica file to the given path.
    /// If the file already exists, it will be overwritten.
    /// Depending on the platform, this function may fail if the full directory path does not exist.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the file.
    ///
    /// # Examples
    ///
    /// ```
    /// let litematica = LitematicaFile::read("path/to/schematic.litematic").unwrap();
    /// litematica.write("path/to/schematic.litematic").unwrap();
    /// ```
    pub fn write(&self, path: impl AsRef<Path>) -> Result<()> {
        let file = File::create(path)?;
        let mut buf_writer = BufWriter::new(file);
        nbt::to_gzip_writer(&mut buf_writer, self, None)?;

        Ok(())
    }

    pub fn get_region_names(&self) -> impl Iterator<Item = &str> {
        self.regions.keys().map(|s| s.as_str())
    }

    pub fn get_region<Q: ?Sized>(&self, name: &Q) -> Option<&Region>
    where
        String: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.regions.get(name)
    }

    pub fn get_region_mut<Q: ?Sized>(&mut self, name: &Q) -> Option<&mut Region>
    where
        String: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.regions.get_mut(name)
    }

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
