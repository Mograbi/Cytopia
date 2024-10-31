use std::collections::HashMap;

use sdl2::render::{TextureCreator, Texture};
use sdl2::video::WindowContext;
use sdl2::image::LoadTexture;

use crate::settings::Settings;
use crate::tile_data::{TileData, TileType};

pub struct TileManager<'a> {
    tile_map: HashMap<String, TileData>,
    texture_map: HashMap<String, Texture<'a>>,
}

impl<'a> TileManager<'a> {
  pub fn init(&mut self, texture_creator: &'a TextureCreator<WindowContext>, settings: &Settings) {
    let tile_list: Vec<TileData> = serde_json::from_str(
        &std::fs::read_to_string(&format!("data/{}", settings.config_files.tile_data_json_file)).unwrap(),
    )
    .unwrap();

    self.tile_map = HashMap::new();
    self.texture_map = HashMap::new();
    for tile in &tile_list {
        // println!("Tile: {}", tile.title);
        self.tile_map.insert(tile.id.clone(), tile.clone());
        // data / filename

        let texture = texture_creator
            .load_texture(&format!("data/{}", tile.tiles.file_name))
            .unwrap();
        self.texture_map.insert(tile.id.clone(), texture);
    }
  }

  pub fn get_tile_data(&self, id: &str) -> Option<&TileData> {
    self.tile_map.get(id)
  }

  pub fn get_texture(&self, id: &str) -> Option<&Texture> {
    self.texture_map.get(id)
  }

  pub fn default() -> Self {
    TileManager {
        tile_map: HashMap::new(),
        texture_map: HashMap::new(),
    }
  } 

  // Constructor with builder pattern
  pub fn builder() -> TileManager<'a> {
      TileManager::default()
  }

  pub fn get_tile_layer(&self, id: &str) -> u8 {
    match self.tile_map.get(id) {
        Some(tile_data) => tile_data.tile_type as u8,
        None => 0,
    }
  }

  pub fn is_auto_tile(&self, id: &str) -> bool {
    let tile_data = self.tile_map.get(id).unwrap();
    match tile_data.tile_type {
        TileType::Road => true,
        TileType::Autotile => true,
        TileType::Underground => true,
        TileType::Powerline => true,
        _ => false,
    }
  }

}