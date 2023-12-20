mod sound;
mod world;

use std::cmp::min;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::sys::{SDL_Delay, SDL_GetTicks64, Uint32, Uint64};
use sdl2::video::Window;

use sound::new_player;
use world::{StepError, StepOk, Thing, World, FIELD_SIZE};

const FRAME_DELTA: Uint64 = 60;
// update screen after the given number of SDL ticks
const WAIT: Uint64 = 20;

struct DrawRect {
    border_x: u32,
    border_y: u32,
    cell: u32,
    canvas: Canvas<Window>,
}

impl DrawRect {
    fn new(field_size: &usize, window: Window) -> Self {
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

        Self {
            border_x,
            border_y,
            cell,
            canvas,
        }
    }
    fn rect(&mut self, x: &u32, y: &u32, c: &Color) {
        let rx = self.border_x + self.cell * *x;
        let ry = self.border_y + self.cell * *y;
        let rect = Rect::new(rx as i32, ry as i32, self.cell, self.cell);
        self.canvas.set_draw_color(*c);
        self.canvas
            .fill_rect(rect)
            .expect("SDL error: cannot draw rectangle");
    }
    fn clear(&mut self) {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();
    }
    fn present(&mut self) {
        self.canvas.present();
    }
    fn window(&self) -> Option<&Window> {
        Some(self.canvas.window())
    }
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("rnake", 0, 0)
        .fullscreen_desktop()
        .build()
        .unwrap();
    let mut draw = DrawRect::new(&FIELD_SIZE, window);

    let maybe_audio_subsystem = sdl_context.audio();
    let sounds = new_player(maybe_audio_subsystem);

    let mut w = World::init();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut next_frame: Uint64 = 0;
    let mut turned = false;

    sounds.play("start");
    sdl2::messagebox::show_simple_message_box(
        sdl2::messagebox::MessageBoxFlag::INFORMATION,
        "Start",
        "Prepare to start the game",
        draw.window(),
    )
    .expect("Should be able to show messagebox");

    'running: loop {
        // process quit and turn the snake events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    if !turned {
                        w.turn_right();
                        turned = true;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    if !turned {
                        w.turn_left();
                        turned = true;
                    }
                }
                _ => {}
            }
        }

        // check if we are at the right moment
        unsafe {
            if SDL_GetTicks64() < next_frame {
                SDL_Delay(min(
                    WAIT as Uint32,
                    (next_frame - SDL_GetTicks64()) as Uint32,
                ));
                continue 'running;
            }
        }

        // Advance
        match w.step() {
            Err(StepError::OutOfField) => {
                sounds.play("wall");
                break 'running;
            }
            Err(StepError::SelfHit) => {
                sounds.play("boom");
                break 'running;
            }
            Ok(StepOk::AteFood) => {
                sounds.play("food");
            }
            Ok(StepOk::Nothing) => {}
        }

        draw.clear();

        // draw field border
        for b in 0..=(FIELD_SIZE as u32 + 1) {
            let wall = &Color::YELLOW;
            draw.rect(&b, &0, wall);
            draw.rect(&b, &(FIELD_SIZE as u32 + 1), wall);
            draw.rect(&0, &b, wall);
            draw.rect(&(FIELD_SIZE as u32 + 1), &b, wall);
        }

        // draw the snake head
        let (hx, hy) = w
            .snake
            .first()
            .expect("Programming error: a snake cannot be empty");
        draw.rect(&(*hx as u32 + 1), &(*hy as u32 + 1), &Color::GREEN);

        // draw rest of the snake
        for (bx, by) in &w.snake[1..] {
            draw.rect(&(*bx as u32 + 1), &(*by as u32 + 1), &Color::GRAY);
        }

        // Draw the things
        for (t, x, y) in &(w.things) {
            let c = match t {
                Thing::Food => Color::BLUE,
            };
            draw.rect(&(*x as u32 + 1), &(*y as u32 + 1), &c);
        }

        draw.present();

        next_frame = unsafe { SDL_GetTicks64() } + FRAME_DELTA;
        turned = false;

        if w.things.is_empty() {
            w.add_thing();
        }
    }
    sdl2::messagebox::show_simple_message_box(
        sdl2::messagebox::MessageBoxFlag::ERROR,
        "Game over",
        "Game over",
        draw.window(),
    )
    .expect("Should be able to show messagebox");
}
