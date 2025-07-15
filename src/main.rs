mod config;
mod ui;

use config::{Config, WindowState};
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use sdl3::render::FRect;
use sdl3::ttf;
use sdl3::video::Window;
use std::time::Duration;

fn get_window_state(window: &Window) -> WindowState {
    let position = Some(window.position());
    let flags = window.window_flags();
    // SDL_WINDOW_MAXIMIZED is typically 0x00000080
    let maximized = (flags & 0x00000080) != 0;
    
    // If maximized, save default size instead of maximized size
    let (width, height) = if maximized {
        (800, 600)
    } else {
        window.size()
    };
    
    WindowState {
        position,
        width,
        height,
        maximized,
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context = sdl3::init()?;
    let video_subsystem = sdl_context.video()?;
    let ttf_context = ttf::init()?;

    // Load config
    let mut config = Config::load();
    
    // Create window with saved state or defaults
    let window = config.create_window_builder(&video_subsystem, "Editor")?.build()?;
    let mut canvas = window.into_canvas();
    let texture_creator = canvas.texture_creator();
    
    // Load a font - using system font path for macOS
    let font_path = "/System/Library/Fonts/Helvetica.ttc";
    let font = ttf_context.load_font(font_path, 64.0)?;
    
    // Render text to surface
    let surface = font
        .render("Hello World!")
        .blended(Color::RGB(255, 255, 255))?;
    
    let texture = texture_creator.create_texture_from_surface(&surface)?;
    let (width, height) = surface.size();
    
    let mut event_pump = sdl_context.event_pump()?;
    
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    // Save window state before exiting
                    let window = canvas.window();
                    config.window_state = get_window_state(window);
                    config.save()?;
                    break 'running;
                },
                _ => {}
            }
        }
        
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        
        // Get current window size
        let (window_width, window_height) = canvas.output_size()?;
        
        // Center the text using FRect for SDL3
        let target = FRect::new(
            ((window_width as i32 - width as i32) / 2) as f32,
            ((window_height as i32 - height as i32) / 2) as f32,
            width as f32,
            height as f32
        );
        
        canvas.copy(&texture, None::<FRect>, target)?;
        canvas.present();
        
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}