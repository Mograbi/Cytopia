use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    render::{TextureCreator, WindowCanvas},
    video::WindowContext,
};
use std::{path::Path, time::Duration};
mod settings;
use settings::Settings;
mod sprite;
mod tile_data;
mod tile_manager;
mod map;
use map::Map;

const TILE_WIDTH: u32 = 32;
const TILE_HEIGHT: u32 = 23;
const TILESET_WIDTH: u32 = 16; // 16 tiles in a row

fn draw_tile(
    canvas: &mut WindowCanvas,
    index: u32,
    x: i32,
    y: i32,
    tile_manager: &tile_manager::TileManager,
    tile_id: &str,
) -> Result<(), String> {
    if index >= TILESET_WIDTH {
        return Err(format!("Invalid tile index: {}", index));
    }

    let texture = tile_manager.get_texture(tile_id).unwrap();
    let tile_x = (13 % TILESET_WIDTH) * TILE_WIDTH;
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
    map: &Map,
    tile_manager: &tile_manager::TileManager,
) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    // Draw greeting
    // let surface = font
    //     .render("Hello, SDL2!")
    //     .blended(Color::RGBA(255, 0, 0, 255))
    //     .map_err(|e| e.to_string())?;
    // let texture = texture_creator
    //     .create_texture_from_surface(&surface)
    //     .map_err(|e| e.to_string())?;
    // let target = Rect::new(10, 0, 200, 100);
    // canvas.copy(&texture, None, target)?;

    // Draw tiles (example)
    for i in 0..128*128 {
        let (x, y) = map.get_tile_coords(i);
        draw_tile(canvas, 0, x * 32, y * 32, tile_manager, &map.get_tile_id(i))?;
    }

    canvas.present();
    Ok(())
}

fn handle_events(event_pump: &mut sdl2::EventPump) -> Result<bool, String> {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }  => return Ok(false),
            Event::MouseMotion { x, y, .. } => {
                println!("Mouse moved: x={}, y={}", x, y);
            }
            Event::MouseButtonDown { x, y, .. } => {
                println!("Mouse button pressed: x={}, y={}", x, y);
            }
            Event::MouseButtonUp { x, y, .. } => {
                println!("Mouse button released: x={}, y={}", x, y);
            }
            Event::KeyDown { timestamp, window_id, keycode, scancode, keymod, repeat } => {
                println!("Key pressed: timestamp={}, window_id={}, keycode={:?}, scancode={:?}, keymod={:?}, repeat={}", timestamp, window_id, keycode, scancode, keymod, repeat);
                match keycode {
                    Some(Keycode::Escape) => return Ok(false),
                    _ => {}
                }
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

    let settings = Settings::load_from_file().expect("Failed to load settings file");
    let screen_width = settings.graphics.get_width();
    let screen_height = settings.graphics.get_height();

    let window = video_subsystem
        .window("Cytopia", screen_width, screen_height)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas: WindowCanvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut tile_manager = tile_manager::TileManager::builder();
    tile_manager.init(&texture_creator, &settings);


    let mut map = Map::builder(settings.game.map_size);
    map.init().expect("Failed to initialize map");

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

        render(&mut canvas, &texture_creator, &font, &map,  &tile_manager)?;

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
