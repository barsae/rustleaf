use sdl3::ttf;

#[allow(dead_code)]
pub struct Font<'ttf> {
    pub font: ttf::Font<'ttf>,
}

impl<'ttf> Font<'ttf> {
    #[allow(dead_code)]
    pub fn load(ttf_context: &'ttf ttf::Sdl3TtfContext, path: &str, size: f32) -> Result<Self, Box<dyn std::error::Error>> {
        let font = ttf_context.load_font(path, size)?;
        Ok(Font { font })
    }
}