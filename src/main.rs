use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::ttf::Font;
use sdl2::render::TextureCreator;
use sdl2::video::{Window, WindowContext};
use std::time::Duration;

mod menu;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

    let window = video_subsystem
        .window("Cytopia", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator: TextureCreator<WindowContext> = canvas.texture_creator();
    let font: Font = ttf_context.load_font("data/resources/fonts/pixelFJ8pt1.ttf", 24)?;

    let mut main_menu = menu::MainMenu::new(800, 600)?;

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::MouseMotion { x, y, .. } => {
                    main_menu.update(x, y);
                }
                _ => {}
            }
        }

        main_menu.render(&mut canvas, &font, &texture_creator)?;
        canvas.present();
        std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
