use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[repr(u8)]
pub enum TileType {
    Default = 0,      ///< Default is for buildings and practically everything that'll be placed on the TERRAIN layer
    Flora = 1,        ///< Flora and Fauna,Trees and so on
    Terrain = 2,      ///< Terrain itself
    Water = 3,        ///< Water terrain
    Blueprint = 4,    ///< Same as terrain, but gets placed on the BLUEPRINT layer
    Autotile = 5,     ///< Autotiling to itself, like roads, power lines, etc
    Zone = 6,         ///< ZoneType (rectangular placement)
    Road = 7,         ///< Roads
    Powerline = 8,    ///< Powerlines
    Grounddecoration = 9, ///< Draw this Tile on GROUNDDECORATION layer. Buildings can be placed over it
    Underground = 10, ///< same as AUTOTILE, but for the BLUEPRINT layer
    Rci = 11,         
}

impl Default for TileType {
    fn default() -> Self {
        TileType::Default
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TileData {
    pub required_tiles: RequiredTiles,
    pub author: String,
    pub category: String,
    pub crime_level: u8,
    pub description: String,
    pub education_level: u8,
    pub fire_hazard_level: u8,
    pub happiness: u8,
    pub id: String,
    pub inhabitants: u16,
    pub is_over_placable: bool,
    pub place_on_ground: bool,
    pub place_on_water: bool,
    pub pollution_level: u8,
    pub power: u16,
    pub price: u8,
    pub sub_category: String,
    pub tags: Option<Vec<String>>,
    pub tile_type: TileType,
    pub tiles: Tiles,
    pub title: String,
    pub upkeep_cost: u8,
    pub water: u8,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequiredTiles {
    pub height: u8,
    pub width: u8,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tiles {
    pub clip_height: u16,
    pub clip_width: u16,
    pub count: u8,
    pub file_name: String,
    pub offset: u8,
    pub pick_random_tile: bool,
}