use sdl3::pixels::Color;
use sdl3::render::{Texture, TextureCreator};
use sdl3::video::WindowContext;
use crate::ui::Font;

pub struct Label {
    text: String,
    color: Color,
}

impl Label {
    pub fn new(text: &str, color: Color) -> Self {
        Label {
            text: text.to_string(),
            color,
        }
    }

    pub fn render<'a>(&self, font: &Font, texture_creator: &'a TextureCreator<WindowContext>) -> Result<Texture<'a>, Box<dyn std::error::Error>> {
        let surface = font.font
            .render(&self.text)
            .blended(self.color)?;
        
        let texture = texture_creator.create_texture_from_surface(&surface)?;
        Ok(texture)
    }
}