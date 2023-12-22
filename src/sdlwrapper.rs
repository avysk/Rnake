use std::cmp::min;

use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureQuery};
use sdl2::rwops::RWops;
use sdl2::sys::{SDL_ShowCursor, SDL_DISABLE};
use sdl2::ttf::{Font, Sdl2TtfContext};
use sdl2::video::Window;
use sdl2::{pixels::Color, EventPump};

use crate::sound::{Player, Sounds};

macro_rules! rect {
    ($x:expr, $y:expr, $w:expr, $h:expr) => {
        Rect::new(($x) as i32, ($y) as i32, ($w) as u32, ($h) as u32)
    };
}

const LINE_INTERVAL: u32 = 10;

pub struct SDLWrapper<'a> {
    // event pump
    pub events: EventPump,
    // graphics
    border_x: u32,
    border_y: u32,
    score_x: u32,
    score_y: u32,
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
        unsafe {
            SDL_ShowCursor(SDL_DISABLE as i32);
        }
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
            .load_font_from_rwops(rwops, 72)
            .expect("Should be able to load font from rwops.");

        Self {
            events,
            border_x,
            border_y,
            score_x: window_size.0 - 150, // TODO
            score_y: 50,
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
    pub fn messages(&mut self, msgs: Vec<&str>) {
        self.clear();
        let surfaces = msgs.iter().map(|line| {
            self.font
                .render(line)
                .solid(Color::BLUE)
                .expect("Should be able to render text line")
        });
        let creator = self.canvas.texture_creator();
        let textures = surfaces.map(|surface| {
            creator
                .create_texture_from_surface(surface)
                .expect("Should be able to create texture from surface")
        });
        let heights: u32 = textures.clone().map(|texture| texture.query().height).sum();
        let total_height = heights + (msgs.len() as u32 - 1) * LINE_INTERVAL;
        let (win_width, win_height) = self
            .window()
            .expect("Should be able to get window corresponding to the texture")
            .size();
        let mut pad_h = (win_height - total_height) / 2;

        for texture in textures {
            let TextureQuery { width, height, .. } = texture.query();
            let tgt = rect!((win_width - width) / 2, pad_h, width, height);
            self.canvas
                .copy(&texture, None, Some(tgt))
                .expect("Should be able to copy texture to canvas.");
            pad_h += height;
            pad_h += LINE_INTERVAL;
        }
        self.present();
    }
    pub fn score(&mut self, sc: u32) {
        let surface = self
            .font
            .render(sc.to_string().as_ref())
            .solid(Color::YELLOW)
            .expect("Should be able to render score");
        let creator = self.canvas.texture_creator();
        let texture = creator
            .create_texture_from_surface(surface)
            .expect("Should be able to create texture from surface");
        let TextureQuery { width, height, .. } = texture.query();
        let tgt = rect!(self.score_x, self.score_y, width, height);
        self.canvas
            .copy(&texture, None, Some(tgt))
            .expect("Should be able to copy texture to canvas");
    }
}
