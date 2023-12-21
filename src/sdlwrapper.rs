use std::cmp::min;

use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureQuery};
use sdl2::rwops::RWops;
use sdl2::ttf::{Font, Sdl2TtfContext};
use sdl2::video::Window;
use sdl2::{pixels::Color, EventPump};

use crate::sound::{Player, Sounds};

macro_rules! rect {
    ($x:expr, $y:expr, $w:expr, $h:expr) => {
        Rect::new(($x) as i32, ($y) as i32, ($w) as u32, ($h) as u32)
    };
}

// const MESSAGE_PADDING: u32 = 50;

pub struct SDLWrapper<'a> {
    // event pump
    pub events: EventPump,
    // graphics
    border_x: u32,
    border_y: u32,
    cell: u32,
    canvas: Canvas<Window>,
    // sound player
    pub sounds: Box<dyn Player>,
    // text
    font: Font<'a, 'static>,
}

impl<'a> SDLWrapper<'a> {
    pub fn new(field_size: &u32, context: &'a Sdl2TtfContext) -> Self {
        let sdl_context = sdl2::init().expect("Should be able to get SDL context");
        // Events
        let events = sdl_context
            .event_pump()
            .expect("Should be able to create SDL event pump");
        let video_subsystem = sdl_context
            .video()
            .expect("Should be able to get SDL video subsystem");
        let window = video_subsystem
            .window("rnake", 0, 0)
            .fullscreen_desktop()
            .build()
            .expect("Should be able to create SDL window");
        let window_size = window.size();
        let canvas = window
            .into_canvas()
            .build()
            .expect("Should be able to get window's canvas");
        let square = min(window_size.0, window_size.1);
        // 2 for the wall around the field
        let field_plus_wall = *field_size + 2;
        // we divide and multiply to round the things
        let cell = square / field_plus_wall;
        let border_x = (window_size.0 - cell * field_plus_wall) / 2;
        let border_y = (window_size.1 - cell * field_plus_wall) / 2;

        // Sounds
        let maybe_audio_subsystem = sdl_context.audio();
        let sounds = Sounds::create(maybe_audio_subsystem);

        // Fonts
        let rwops = RWops::from_bytes(include_bytes!("fonts/Aclonica.ttf"))
            .expect("Should be able to load rwops from font bytes.");
        let font = context
            .load_font_from_rwops(rwops, 36)
            .expect("Should be able to load font from rwops.");

        Self {
            events,
            border_x,
            border_y,
            cell,
            canvas,
            sounds,
            font,
        }
    }
    pub fn rect(&mut self, x: &u32, y: &u32, c: &Color) {
        let rx = self.border_x + self.cell * *x;
        let ry = self.border_y + self.cell * *y;
        let rect = rect!(rx, ry, self.cell, self.cell);
        self.canvas.set_draw_color(*c);
        self.canvas
            .fill_rect(rect)
            .expect("SDL error: cannot draw rectangle");
    }
    pub fn clear(&mut self) {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();
    }
    pub fn present(&mut self) {
        self.canvas.present();
    }
    pub fn window(&self) -> Option<&Window> {
        Some(self.canvas.window())
    }
    pub fn message(&mut self, msg: &str) {
        self.clear();
        let surface = self
            .font
            .render(msg)
            .solid(Color::BLUE)
            .expect("Should be able to render message");
        let creator = self.canvas.texture_creator();
        let texture = creator
            .create_texture_from_surface(surface)
            .expect("Should be able to create texture from surface");
        let TextureQuery { width, height, .. } = texture.query();

        // TODO: check that we fit into the screen with the given padding

        let s = self
            .window()
            .expect("Should be able to get window corresponding to the canvas")
            .size();
        let msg_x = (s.0 - width) / 2;
        let msg_y = (s.1 - height) / 2;

        self.canvas
            .copy(&texture, None, Some(rect!(msg_x, msg_y, width, height)))
            .expect("Should be able to copy texture to canvas.");
        self.present();
    }
}
