use std::{collections::HashMap, env, path::Path, time::Duration};

use sdl2::{
    event::Event,
    image::LoadTexture,
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    render::{Texture, TextureCreator, WindowCanvas},
    video::WindowContext,
};

mod settings;
mod tile_data;
mod sprite;

use tile_data::TileData;
use rand::Rng;

const TILE_WIDTH: u32 = 32;
const TILE_HEIGHT: u32 = 23;
const TILESET_WIDTH: u32 = 16; // 16 tiles in a row

use noise::{Perlin, NoiseFn};

struct NoiseMap {
    noise_map: Vec<Vec<f64>>,
}

impl NoiseMap {
    fn generate(&mut self, size_x: usize, size_y: usize, random_seed: u32) {
        self.noise_map = Vec::new();

        let noise1 = Perlin::new(random_seed);
        let noise2 = Perlin::new(random_seed);
        let noise3 = Perlin::new(random_seed);
        let noise4 = Perlin::new(random_seed);

        let xpix = size_x + 1;
        let ypix = size_y + 1;
        for j in 0..ypix {
            let mut row = Vec::new();
            for i in 0..xpix {
                let mut noise_val = noise1.get([i as f64 / xpix as f64, j as f64 / ypix as f64]);
                noise_val += 0.5 * noise2.get([i as f64 / xpix as f64, j as f64 / ypix as f64]);
                noise_val += 0.25 * noise3.get([i as f64 / xpix as f64, j as f64 / ypix as f64]);
                noise_val += 0.125 * noise4.get([i as f64 / xpix as f64, j as f64 / ypix as f64]);
                row.push(noise_val);
            }
            self.noise_map.push(row);
        }
    }
}

fn get_tile_map(texture_creator: &TextureCreator<WindowContext>) -> HashMap<String, TileData> {
    let tile_list: Vec<TileData> = serde_json::from_str(
        &std::fs::read_to_string("data/resources/data/tileData.json").unwrap(),
    )
    .unwrap();

    let mut tile_data = HashMap::new();
    let mut texture_map: HashMap<String, Texture> = HashMap::new();
    for tile in &tile_list {
        println!("Tile: {}", tile.title);
        tile_data.insert(tile.id.clone(), tile.clone());
        // data / filename

        let texture = texture_creator.load_texture(&format!("data/{}", tile.tiles.file_name)).unwrap();
        texture_map.insert(tile.id.clone(), texture);
    }

    tile_list
        .into_iter()
        .map(|tile| (tile.id.clone(), tile))
        .collect()
}

fn draw_tile(
    canvas: &mut WindowCanvas,
    texture_creator: &TextureCreator<WindowContext>,
    index: u32,
    x: i32,
    y: i32,
) -> Result<(), String> {
    if index >= TILESET_WIDTH {
        return Err(format!("Invalid tile index: {}", index));
    }

    let texture =
        texture_creator.load_texture("data/resources/images/terrain/terrain_grass.png")?;

    let tile_x = (index % TILESET_WIDTH) * TILE_WIDTH;
    let tile_y = 0; // All tiles are in the first row

    let src_rect = Rect::new(tile_x as i32, tile_y as i32, TILE_WIDTH, TILE_HEIGHT);
    let dest_rect = Rect::new(x, y, TILE_WIDTH, TILE_HEIGHT);

    canvas.copy(&texture, src_rect, dest_rect)?;

    Ok(())
}

fn render(
    canvas: &mut WindowCanvas,
    texture_creator: &TextureCreator<WindowContext>,
    font: &sdl2::ttf::Font,
    tile_map: &HashMap<String, TileData>,
) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    // Draw greeting
    let surface = font
        .render("Hello, SDL2!")
        .blended(Color::RGBA(255, 0, 0, 255))
        .map_err(|e| e.to_string())?;
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;
    let target = Rect::new(10, 0, 200, 100);
    canvas.copy(&texture, None, target)?;

    // Draw tiles (example)
    draw_tile(canvas, texture_creator, 0, 370, 277)?;
    draw_tile(canvas, texture_creator, 1, 403, 277)?;

    canvas.present();
    Ok(())
}

fn handle_events(event_pump: &mut sdl2::EventPump) -> Result<bool, String> {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => return Ok(false),
            Event::MouseMotion { x, y, .. } => {
                println!("Mouse moved: x={}, y={}", x, y);
            }
            Event::MouseButtonDown { x, y, .. } => {
                println!("Mouse button pressed: x={}, y={}", x, y);
            }
            Event::MouseButtonUp { x, y, .. } => {
                println!("Mouse button released: x={}, y={}", x, y);
            }
            _ => {}
        }
    }
    Ok(true)
}

// --- Main Function ---

fn main() -> Result<(), String> {
    let context = sdl2::init()?;
    let video_subsystem = context.video()?;

    let settings_path = env::current_dir()
        .unwrap()
        .join("data/resources/settings.json");
    let settings = settings::Settings::load_from_file(
        settings_path.to_str().ok_or("Invalid path")?, // More informative error
    )
    .unwrap();
    let screen_width = settings.graphics.get_width();
    let screen_height = settings.graphics.get_height();

    let window = video_subsystem
        .window("Cytopia", screen_width, screen_height)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas: WindowCanvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    let tile_map = get_tile_map(&texture_creator);

    let mut noise_map = NoiseMap {
        noise_map: Vec::new(),
    };
    let random_seed: u32 = Rng::gen(&mut rand::thread_rng());
    noise_map.generate(128, 128, random_seed);

    dbg!(noise_map.noise_map);

    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let font_path: &Path = Path::new(&"data/resources/fonts/pixelFJ8pt1.ttf");
    let mut font = ttf_context
        .load_font(font_path, 128)
        .map_err(|e| e.to_string())?;
    font.set_style(sdl2::ttf::FontStyle::BOLD);

    let mut event_pump = context.event_pump()?;

    'running: loop {
        if !handle_events(&mut event_pump)? {
            break 'running;
        }

        render(&mut canvas, &texture_creator, &font, &tile_map)?;

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
