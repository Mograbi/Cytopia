use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    render::{TextureCreator, WindowCanvas},
    video::WindowContext,
    mouse::MouseButton,
};
use std::{path::Path, time::Duration};
mod settings;
use settings::Settings;
mod sprite;
mod tile_data;
mod tile_manager;
mod map;
mod input_manager;
mod camera;
use map::Map;
use camera::Camera;

const TILE_WIDTH: u32 = 32;
const TILE_HEIGHT: u32 = 23;
const TILESET_WIDTH: u32 = 16; // 16 tiles in a row

use std::ops::{Mul, Add};

#[derive(Debug, Clone, Copy)]
struct Vector2 {
    x: f32,
    y: f32,
}

impl Mul<f32> for Vector2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Add for Vector2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

// These are the four numbers that define the transform, i hat and j hat
const I_X: f32 = 1.0;
const I_Y: f32 = 0.5;
const J_X: f32 = -1.0;
const J_Y: f32 = 0.5;
// Sprite size
const W: f32 = 32.0;
const H: f32 = 32.0;

fn to_screen_coordinate(tile: Vector2) -> Vector2 {
    // For isometric tiles to connect properly, we need to position them closer together
    // We'll use the exact tile dimensions to ensure tiles stay connected
    Vector2 {
        x: (tile.x - tile.y) * TILE_WIDTH as f32 * 0.5,
        y: (tile.x + tile.y) * TILE_HEIGHT as f32 * 0.25,
    }
}

// Going from screen coordinate to grid coordinate
fn to_grid_coordinate(screen: Vector2) -> Vector2 {
    let x = screen.x / (TILE_WIDTH as f32 * 0.5);
    let y = screen.y / (TILE_HEIGHT as f32 * 0.25);
    Vector2 {
        x: (x + y) / 2.0,
        y: (y - x) / 2.0,
    }
}

fn draw_tile(
    canvas: &mut WindowCanvas,
    index: u32,
    x: i32,
    y: i32,
    tile_manager: &tile_manager::TileManager,
    tile_id: &str,
    settings: &Settings,
    camera: &Camera,
) -> Result<(), String> {
    if index >= TILESET_WIDTH {
        return Err(format!("Invalid tile index: {}", index));
    }

    let texture = tile_manager.get_texture(tile_id).unwrap();
    let tile_x = (13 % TILESET_WIDTH) * TILE_WIDTH;
    let tile_y = 0;

    // Calculate base isometric position
    let vec: Vector2 = to_screen_coordinate(Vector2 { x: x as f32, y: y as f32 });
    let src_rect = Rect::new(tile_x as i32, tile_y as i32, TILE_WIDTH, TILE_HEIGHT);
    
    let (window_width, window_height) = canvas.output_size().unwrap();
    let center_x = (window_width / 2) as f32;
    let center_y = (window_height / 3) as f32;
    
    // Scale the position by zoom before adding camera offset
    let scaled_x = vec.x * camera.zoom;
    let scaled_y = vec.y * camera.zoom;
    
    let dest_rect = Rect::new(
        (scaled_x + center_x + camera.x) as i32,
        (scaled_y + center_y + camera.y) as i32,
        (TILE_WIDTH as f32 * camera.zoom) as u32,
        (TILE_HEIGHT as f32 * camera.zoom) as u32
    );

    canvas.copy(&texture, src_rect, dest_rect)?;

    Ok(())
}

fn render(
    canvas: &mut WindowCanvas,
    settings: &Settings,
    map: &Map,
    tile_manager: &tile_manager::TileManager,
    camera: &Camera,
) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    for i in 0..settings.game.map_size * settings.game.map_size {
        let (x, y) = map.get_tile_coords(i);
        draw_tile(canvas, 0, x, y, tile_manager, &map.get_tile_id(i), settings, camera)?;
    }

    canvas.present();
    Ok(())
}

fn handle_events(event_pump: &mut sdl2::EventPump, camera: &mut Camera, window_width: u32, window_height: u32) -> Result<bool, String> {
    let mouse_state = event_pump.mouse_state();
    let mouse_x = mouse_state.x();
    let mouse_y = mouse_state.y();

    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } => return Ok(false),
            Event::MouseMotion { x, y, .. } => {
                camera.update_drag(x, y);
            }
            Event::MouseButtonDown { x, y, mouse_btn, .. } => {
                if mouse_btn == MouseButton::Right {
                    camera.start_drag(x, y);
                }
            }
            Event::MouseButtonUp { mouse_btn, .. } => {
                if mouse_btn == MouseButton::Right {
                    camera.stop_drag();
                }
            }
            Event::MouseWheel { y, .. } => {
                let zoom_delta = if y > 0 { 1.1 } else if y < 0 { 0.9 } else { 1.0 };
                if zoom_delta != 1.0 {
                    let center_x = window_width as i32 / 2;
                    let center_y = window_height as i32 / 3;
                    
                    // Adjust mouse position relative to the center
                    let adjusted_x = mouse_x - center_x;
                    let adjusted_y = mouse_y - center_y;
                    
                    camera.zoom_at_point(adjusted_x, adjusted_y, zoom_delta);
                }
            }
            Event::KeyDown { keycode, .. } => {
                if let Some(Keycode::Escape) = keycode {
                    return Ok(false);
                }
            }
            _ => {}
        }
    }
    Ok(true)
}

// --- Main Function ---

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let settings = Settings::load_from_file().expect("Failed to load settings file");
    let screen_width = settings.graphics.get_width();
    let screen_height = settings.graphics.get_height();

    let window = video_subsystem
        .window("Cytopia", screen_width, screen_height)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
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

    let mut event_pump = sdl_context.event_pump()?;
    let mut camera = Camera::new();

    'running: loop {
        if !handle_events(&mut event_pump, &mut camera, screen_width, screen_height)? {
            break 'running;
        }

        render(&mut canvas, &settings, &map, &tile_manager, &camera)?;

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
