use std::cmp::min;

use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::{pixels::Color, EventPump};

use crate::sound::{Player, Sounds};

pub struct SDLWrapper {
    // event pump
    pub events: EventPump,
    // graphics
    border_x: u32,
    border_y: u32,
    cell: u32,
    canvas: Canvas<Window>,
    // sound player
    pub sounds: Box<dyn Player>,
}

impl SDLWrapper {
    pub fn new(field_size: &usize) -> Self {
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
        let field_plus_wall = *field_size as u32 + 2;
        // we divide and multiply to round the things
        let cell = square / field_plus_wall;
        let border_x = (window_size.0 - cell * field_plus_wall) / 2;
        let border_y = (window_size.1 - cell * field_plus_wall) / 2;

        // Sounds
        let maybe_audio_subsystem = sdl_context.audio();
        let sounds = Sounds::create(maybe_audio_subsystem);

        Self {
            events,
            border_x,
            border_y,
            cell,
            canvas,
            sounds,
        }
    }
    pub fn rect(&mut self, x: &u32, y: &u32, c: &Color) {
        let rx = self.border_x + self.cell * *x;
        let ry = self.border_y + self.cell * *y;
        let rect = Rect::new(rx as i32, ry as i32, self.cell, self.cell);
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
}
