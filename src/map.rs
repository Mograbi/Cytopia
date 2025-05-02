use std::time::{SystemTime, UNIX_EPOCH};
use noise::{NoiseFn, Perlin, OpenSimplex};

use noise::ScaleBias;
use rand::Error;

struct MapNode {
    x: u32,
    y: u32,
    z: u32,
    tile_id: String,
}

pub struct Map {
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<u32>,
    pub nodes: Vec<MapNode>,
    seed: u32
}

impl Map {
    pub fn builder(map_size: u32) -> Map {
        Map {
            width: map_size,
            height: map_size,
            tiles: Vec::new(),
            nodes: Vec::new(),
            seed: 0
        }
    }

    pub fn get_tile_id(&self, i: u32) -> String {
        self.nodes[i as usize].tile_id.clone()
    }

    pub fn get_tile_coords(&self, i: u32) -> (i32, i32) {
        (self.nodes[i as usize].x as i32, self.nodes[i as usize].y as i32)
    }

    pub fn init(&mut self) -> Result<(), Error> {
        self.seed = Map::generate_seed().expect("Failed to generate seed"); 
        let perlin = Perlin::new(self.seed);
        
        // Use a larger scale for broader features (fewer, larger lakes)
        let base_scale = 0.03;
        // Secondary noise for detail variation
        let detail_scale = 0.08;

        // Pre-generate the height map
        let mut height_map = vec![0.0; (self.width * self.height) as usize];
        
        // Generate base terrain
        for i in 0..self.width * self.height {
            let y = i / self.width;
            let x = i % self.width;
            
            // Combine two noise layers
            let base_height = perlin.get([x as f64 * base_scale, y as f64 * base_scale]);
            let detail = perlin.get([x as f64 * detail_scale + 1000.0, y as f64 * detail_scale + 1000.0]) * 0.3;
            
            // Combine and normalize height
            let combined_height = (base_height + detail + 1.0) / 2.0;
            height_map[i as usize] = combined_height;
        }

        // Process the height map to create more defined lakes
        for i in 0..self.width * self.height {
            let y = (i / self.width) as i32;
            let x = (i % self.width) as i32;
            let idx = i as usize;

            // Make lakes more defined by creating sharper transitions
            let height = height_map[idx];
            let modified_height = if height < 0.2 {
                // Create deeper lakes
                height * 0.5
            } else if height < 0.25 {
                // Create steeper shores
                0.3 + (height - 0.2) * 2.0
            } else {
                height
            };

            // Determine terrain type based on modified height
            let tile_id = match modified_height {
                h if h < 0.15 => "water",                // Deep lakes
                h if h < 0.25 => "terrain_dirt",         // Shoreline/Beach
                h if h < 0.7 => "terrain_soil",          // Plains/Grass
                h if h < 0.85 => {                       // Forest areas
                    if (x + y) % 2 == 0 {
                        "tree_bamboo_dense"
                    } else {
                        "tree_Cottontop_medium"
                    }
                }
                _ => "terrain_snow",                     // Mountain peaks
            };

            self.nodes.push(MapNode {
                x: x as u32,
                y: y as u32,
                z: (modified_height * 10.0) as u32,
                tile_id: tile_id.to_string(),
            });
        }

        Ok(())
    }

    fn generate_seed() -> Result<u32, Error> {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .subsec_nanos();
        Ok(nanos)
    }

    pub fn new(width: u32, height: u32) -> Result<Self, String> {
        let mut tiles = vec![0; (width * height) as usize];
        let noise = OpenSimplex::new(42);

        for y in 0..height {
            for x in 0..width {
                let nx = x as f64 / width as f64;
                let ny = y as f64 / height as f64;
                let value = noise.get([nx, ny]);
                let tile_index = if value > 0.0 { 1 } else { 0 };
                tiles[(y * width + x) as usize] = tile_index;
            }
        }

        Ok(Self {
            width,
            height,
            tiles,
            nodes: Vec::new(),
            seed: 0
        })
    }

    pub fn get_tile(&self, x: i32, y: i32) -> Option<u32> {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            None
        } else {
            Some(self.tiles[(y * self.width as i32 + x) as usize])
        }
    }
}