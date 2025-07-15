mod config;

use config::{Config, WindowState};
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use sdl3::render::FRect;
use sdl3::ttf;
use sdl3::video::Window;
use std::time::Duration;

fn get_window_state(window: &Window) -> WindowState {
    let (x, y) = window.position();
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
        x,
        y,
        width,
        height,
        maximized,
        display_index: None, // We'll update this separately
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context = sdl3::init()?;
    let video_subsystem = sdl_context.video()?;
    let ttf_context = ttf::init()?;

    // Load config
    let mut config = Config::load();
    
    // Get available displays
    let displays = video_subsystem.displays()?;
    println!("Available displays: {}", displays.len());
    
    // Use saved display index if valid, otherwise use default logic
    let display_index = if let Some(saved_index) = config.window_state.display_index {
        if saved_index < displays.len() {
            saved_index
        } else {
            if displays.len() > 1 { 1 } else { 0 }
        }
    } else {
        if displays.len() > 1 { 1 } else { 0 }
    };
    
    config.window_state.display_index = Some(display_index);
    
    // Create window with saved state or defaults
    let mut window_builder = video_subsystem
        .window("Hello World", config.window_state.width, config.window_state.height);
    
    // Position window based on saved state or center on display
    if config.window_state.x != 100 || config.window_state.y != 100 {
        window_builder.position(config.window_state.x, config.window_state.y);
    } else {
        let display = &displays[display_index];
        let display_bounds = display.get_bounds()?;
        window_builder.position(
            display_bounds.x() + (display_bounds.width() as i32 - config.window_state.width as i32) / 2,
            display_bounds.y() + (display_bounds.height() as i32 - config.window_state.height as i32) / 2
        );
    }
    
    window_builder.resizable();
    
    if config.window_state.maximized {
        window_builder.maximized();
    }
    
    let window = window_builder.build()?;
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
                    config.window_state.display_index = Some(display_index);
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