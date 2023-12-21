use sdl2::rwops::RWops;

pub struct Font<'a> {
    pub font: sdl2::ttf::Font<'a, 'static>,
}

impl<'a> Font<'a> {
    pub fn new(context: &'a sdl2::ttf::Sdl2TtfContext) -> Self {
        let rwops = RWops::from_bytes(include_bytes!("fonts/Aclonica.ttf"))
            .expect("Should be able to load rwops from font bytes.");
        let font = context
            .load_font_from_rwops(rwops, 32)
            .expect("Should be able to load font from rwops.");
        Self { font }
    }
}
