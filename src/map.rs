use std::time::{SystemTime, UNIX_EPOCH};
use noise::{NoiseFn, Perlin};

use noise::ScaleBias;
use rand::Error;

struct MapNode {
    x: u16,
    y: u16,
    z: u16,
    tile_id: String,
}

pub struct Map {
    width: u16,
    height: u16,
    pub nodes: Vec<MapNode>,
    seed: u32
}

impl Map {
    pub fn builder(map_size: u16) -> Map {
        Map {
            width: map_size,
            height: map_size,
            nodes: Vec::new(),
            seed: 0
        }
    }

    pub fn get_tile_id(&self, i: i32) -> String {
        self.nodes[i as usize].tile_id.clone()
    }

    pub fn get_tile_coords(&self, i: i32) -> (i32, i32) {
        (self.nodes[i as usize].x as i32, self.nodes[i as usize].y as i32)
    }

    pub fn init(&mut self) -> Result<(), Error> {
        self.seed = Map::generate_seed().expect("Failed to generate seed"); 
        let mut terrain_height_perlin = Perlin::new(self.seed);

        let mut terrain_height_perlin_scaled: ScaleBias<f64, &Perlin, 2> = noise::ScaleBias::new(&terrain_height_perlin);
        terrain_height_perlin_scaled = terrain_height_perlin_scaled.set_scale(0.5);
        terrain_height_perlin_scaled = terrain_height_perlin_scaled.set_bias(-0.5);

        for i in 0..self.width * self.height {
            let y = i / self.width;
            let x = i % self.width;
            let raw_height = terrain_height_perlin.get([x as f64 * 32.0, y as f64 * 32.0, 0.5]);
            if raw_height < 3.0 {
                self.nodes.push(MapNode {
                    x,
                    y,
                    z: 0,
                    tile_id: "water".to_string(),
                });
            } else {
                self.nodes.push(MapNode {
                    x,
                    y,
                    z: 0,
                    tile_id: "terrain_grass".to_string(),
                });
            }
        };

        Ok(())
    }

    fn generate_seed() -> Result<u32, Error> {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .subsec_nanos();
        Ok(nanos)
    }
}