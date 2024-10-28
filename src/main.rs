use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::{Rect};
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;
use sdl2::Sdl;
use tile_data::TileData;
use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::time::Duration;

mod settings;
mod tile_data;

const TILE_WIDTH: u32 = 32;
const TILE_HEIGHT: u32 = 23;
const TILESET_WIDTH: u32 = 16; // 16 tiles in a row

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

    let texture = texture_creator.load_texture("data/resources/images/terrain/terrain_grass.png")?;

    let tile_x = (index % TILESET_WIDTH) * TILE_WIDTH;
    let tile_y = 0; // All tiles are in the first row

    let mut src_rect = Rect::new(tile_x as i32, tile_y as i32, TILE_WIDTH, TILE_HEIGHT);
    let mut dest_rect = Rect::new(x, y, TILE_WIDTH, TILE_HEIGHT);

    canvas.copy(&texture, src_rect, dest_rect).ok().unwrap();

    // canvas.copy(tileset, Some(src_rect), Some(dest_rect))?;

    Ok(())
}

fn render(
    canvas: &mut WindowCanvas,
    texture_creator: &TextureCreator<WindowContext>,
    font: &sdl2::ttf::Font,
    tile_map: &HashMap<String, TileData>,
) -> Result<(), String> {
    let color = Color::RGB(0, 0, 0);
    canvas.set_draw_color(color);
    canvas.clear();

    // draw greeting
    let hello_text: String = "Hello, SDL2!".to_string();
    let surface = font
        .render(&hello_text)
        .blended(Color::RGBA(255, 0, 0, 255))
        .map_err(|e| e.to_string())?;

    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;

    let target = Rect::new(10, 0, 200, 100);
    canvas.copy(&texture, None, target)?;
    draw_tile(canvas, texture_creator, 0, 370, 277);
    draw_tile(canvas, texture_creator, 1, 403, 277);

    canvas.present();
    Ok(())
}

fn get_tile_map() -> HashMap<String, TileData> {
    let tile_list: Vec<TileData> = serde_json::from_str(
        &std::fs::read_to_string("data/resources/data/tileData.json").unwrap(),
    ).unwrap();

    let mut tile_map: HashMap<String, TileData> = HashMap::new();
    for tile in tile_list {
        tile_map.insert(tile.id.clone(), tile);
    }
    tile_map
}

fn main() -> Result<(), String> {
    let context: Sdl = sdl2::init().unwrap();
    let video_subsystem = context.video().unwrap();

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

    let tile_map = get_tile_map();

    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let font_path: &Path = Path::new(&"data/resources/fonts/pixelFJ8pt1.ttf");
    let mut font = ttf_context
        .load_font(font_path, 128)
        .map_err(|e| e.to_string())?;
    font.set_style(sdl2::ttf::FontStyle::BOLD);

    let mut event_pump = context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    if keycode == sdl2::keyboard::Keycode::Escape {
                        break 'running;
                    }
                }
                Event::MouseMotion {
                    timestamp,
                    window_id,
                    which,
                    mousestate,
                    x,
                    y,
                    xrel,
                    yrel,
                } => {
                    println!("Mouse moved: x={}, y={}", x, y);
                }
                Event::MouseButtonDown {
                    timestamp,
                    window_id,
                    which,
                    mouse_btn,
                    clicks,
                    x,
                    y,
                } => {
                    println!("Mouse button pressed: x={}, y={}", x, y);
                }
                Event::MouseButtonUp {
                    timestamp,
                    window_id,
                    which,
                    mouse_btn,
                    clicks,
                    x,
                    y,
                } => {
                    println!("Mouse button released: x={}, y={}", x, y);
                }
                Event::Quit { .. } => {
                    break 'running;
                }
                _ => {}
            }
        }
        render(&mut canvas, &texture_creator, &font, &tile_map)?;

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}
