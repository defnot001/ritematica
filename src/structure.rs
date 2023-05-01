use serde::{Deserialize, Serialize};
use serde_json::Value;

use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LitematicaFile {
    pub metadata: Metadata,
    pub minecraft_data_version: i32,
    pub version: i32,
    pub(crate) regions: HashMap<String, Region>,
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
    pub(crate) block_state_palette: Vec<BlockState>,

    #[serde(serialize_with = "nbt::i64_array")]
    pub(crate) block_states: Vec<i64>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct BlockState {
    pub(crate) name: String,

    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub(crate) properties: HashMap<String, String>,
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

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash)]
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
