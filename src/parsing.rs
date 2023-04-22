use flate2::bufread::GzDecoder;
use flate2::write::GzEncoder;
use nbt::{from_gzip_reader, Result as NBTResult};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

const BIT_TO_LONG_SHIFT: u8 = 6; //log2(64)

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LitematicaFile {
    pub metadata: Metadata,
    pub minecraft_data_version: i32,
    pub version: i32,
    pub regions: HashMap<String, Region>,
}

impl LitematicaFile {
    pub fn read(path: impl AsRef<Path>) -> crate::error::Result<LitematicaFile> {
        let file = File::open(path)?;
        let buf_reader = BufReader::new(file);
        Ok(nbt::from_gzip_reader(buf_reader)?)
    }

    pub fn write(&self, path: impl AsRef<Path>) -> crate::error::Result<()> {
        let file = File::create(path)?;
        let mut buf_writer = BufWriter::new(file);
        nbt::to_gzip_writer(&mut buf_writer, self, None)?;

        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Coordinates {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl From<(i32, i32, i32)> for Coordinates {
    fn from((x, y, z): (i32, i32, i32)) -> Self {
        Coordinates { x, y, z }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Region {
    pub position: Coordinates,
    pub size: Coordinates,
    pub entities: Vec<Entity>,
    pub tile_entities: Vec<Value>,
    pub pending_block_ticks: Vec<Value>,
    pub pending_fluid_ticks: Vec<Value>,

    #[serde(serialize_with = "nbt::i64_array")]
    pub(crate) block_states: Vec<i64>,
    pub(crate) block_state_palette: Vec<BlockState>,
}

impl Region {
    fn get_index(&self, coords: Coordinates) -> u64 {
        assert!(coords.x >= 0 && coords.x < self.size.x.abs());
        assert!(coords.y >= 0 && coords.y < self.size.y.abs());
        assert!(coords.z >= 0 && coords.z < self.size.z.abs());

        let x = coords.x as u64;
        let y = coords.y as u64;
        let z = coords.z as u64;

        let size_x = self.size.x.abs() as u64;
        let size_layer = size_x * self.size.z.abs() as u64;
        y * size_layer + z * size_x + x
    }

    fn get_block_idx(&self, index: u64, bits: u64, mask: u32) -> u32 {
        let bit_index = index * bits;
        let word_index = (bit_index >> BIT_TO_LONG_SHIFT) as usize;
        let end_word_index = (((index + 1) * bits - 1) >> BIT_TO_LONG_SHIFT) as usize;
        let index_in_word = (bit_index ^ ((word_index as u64) << BIT_TO_LONG_SHIFT)) as u8;

        if word_index == end_word_index {
            (self.block_states[word_index] >> index_in_word) as u32 & mask
        } else {
            let first_bits = 64 - index_in_word;
            ((self.block_states[word_index] >> index_in_word) as u32 & mask)
                | ((self.block_states[end_word_index] << first_bits) as u32 & mask)
        }
    }

    pub fn get_block(&self, position: impl Into<Coordinates>) -> &BlockState {
        let position = position.into();
        let index = self.get_index(position);

        let bits = self
            .block_state_palette
            .len()
            .next_power_of_two()
            .trailing_zeros()
            .max(2) as u64;

        let mask = (1 << bits) - 1;

        let palette_index = self.get_block_idx(index, bits, mask);

        &self.block_state_palette[palette_index as usize]
    }

    pub fn set_block(&mut self, position: impl Into<Coordinates>, block: BlockState) {
        let position = position.into();
        let index = self.get_index(position);

        let mut bits = self
            .block_state_palette
            .len()
            .next_power_of_two()
            .trailing_zeros()
            .max(2) as u64;

        let mut mask = (1 << bits) - 1;

        let palette_index = self
            .block_state_palette
            .iter()
            .position(|b| *b == block)
            .unwrap_or_else(|| {
                let index = self.block_state_palette.len();

                // minimum size is 2 bits
                if index.is_power_of_two() && index >= 4 {
                    let new_bits = bits + 1;
                    let new_mask = (1 << new_bits) - 1;

                    self.resize(bits, mask, new_bits, new_mask);

                    bits = new_bits;
                    mask = new_mask;
                }

                self.block_state_palette.push(block);
                index
            });

        Self::set_block_idx(
            &mut self.block_states,
            index,
            palette_index as u32,
            bits,
            mask,
        );
    }

    fn resize(&mut self, old_bits: u64, old_mask: u32, new_bits: u64, new_mask: u32) {
        let volume = self.size.x.abs() as u64 * self.size.y.abs() as u64 * self.size.z.abs() as u64;
        let required = (volume * new_bits + 63) >> BIT_TO_LONG_SHIFT; // rounding up

        let mut new_states: Vec<i64> = vec![0; required as usize];

        for i in 0..volume {
            let old_value = self.get_block_idx(i, old_bits, old_mask);
            Self::set_block_idx(&mut new_states, i, old_value, new_bits, new_mask);
        }

        self.block_states = new_states;
    }

    fn set_block_idx(block_states: &mut [i64], index: u64, value: u32, bits: u64, mask: u32) {
        let bit_index = index * bits;
        let word_index = (bit_index >> BIT_TO_LONG_SHIFT) as usize;
        let end_word_index = (((index + 1) * bits - 1) >> BIT_TO_LONG_SHIFT) as usize;
        let index_in_word = (bit_index ^ ((word_index as u64) << BIT_TO_LONG_SHIFT)) as u8;

        block_states[word_index] = (block_states[word_index] & !((mask as i64) << index_in_word))
            | (((value & mask) as i64) << index_in_word);

        if word_index != end_word_index {
            let bits_written = 64 - index_in_word;
            let bits_to_write = bits as u8 - bits_written;

            block_states[end_word_index] = (block_states[end_word_index]
                & !((1 << bits_to_write) - 1))
                | ((value & mask) >> bits_written) as i64;
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct BlockState {
    pub name: String,

    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub properties: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Metadata {
    pub author: String,
    pub enclosing_size: Coordinates,
    pub total_volume: i32,
    pub region_count: i32,
    pub description: String,
    pub name: String,
    pub time_modified: i64,
    pub total_blocks: i32,
    pub time_created: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Entity {
    #[serde(rename = "Rotation")]
    pub rotation: Vec<f64>,

    #[serde(rename = "Fire")]
    pub fire: i16,

    #[serde(rename = "Pos")]
    pub pos: Vec<f64>,

    #[serde(rename = "Motion")]
    pub motion: Vec<f64>,

    #[serde(rename = "Air")]
    pub air: i16,

    #[serde(rename = "FallDistance")]
    pub fall_distance: f64,

    #[serde(rename = "OnGround")]
    pub on_ground: bool,

    pub id: String,

    #[serde(rename = "PortalCooldown")]
    pub portal_cooldown: i32,

    #[serde(rename = "UUID")]
    pub uuid: Vec<i32>,

    #[serde(rename = "Invulnerable")]
    pub invulnerable: bool,
}
