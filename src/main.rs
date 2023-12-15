mod sound;
mod world;
use std::cmp::min;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::sys::{SDL_Delay, SDL_GetTicks64, Uint32, Uint64};

use crate::sound::Sounds;
use crate::world::{Thing, World, HEIGHT, WIDTH};

const FRAME_DELTA: Uint64 = 60;
// update screen after the given number of SDL ticks
const WAIT: Uint64 = 20;

const WIN_WIDTH: u32 = 600;
const WIN_HEIGHT: u32 = 600;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let audio_subsystem = sdl_context.audio().unwrap();
    let sounds = Sounds::new(audio_subsystem);

    let window = video_subsystem
        .window("rnake", WIN_WIDTH, WIN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut w = World::init(sounds);
    let cell_width = (WIN_WIDTH as i32) / (WIDTH as i32);
    let cell_height = (WIN_HEIGHT as i32) / (HEIGHT as i32);

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut next_frame: Uint64 = 0;
    let mut turned = false;

    w.play("start");
    sdl2::messagebox::show_simple_message_box(
        sdl2::messagebox::MessageBoxFlag::INFORMATION,
        "Start",
        "Prepare to start the game",
        Some(&window),
    )
    .expect("Should be able to show messagebox");

    let mut canvas = window.into_canvas().build().unwrap();
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
        if w.step().is_err() {
            break 'running;
        }

        // Clear the field
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        // draw the snake head
        canvas.set_draw_color(Color::GREEN);
        let (hx, hy) = w
            .snake
            .first()
            .expect("Programming error: a snake cannot be empty");
        let r = Rect::new(
            cell_width * (*hx as i32),
            cell_height * (*hy as i32),
            cell_width as u32,
            cell_height as u32,
        );
        canvas
            .fill_rect(r)
            .expect("SDL error: cannot draw rectangle");

        // draw rest of the snake
        canvas.set_draw_color(Color::GRAY);
        for (x, y) in &w.snake[1..] {
            let r = Rect::new(
                cell_width * (*x as i32),
                cell_height * (*y as i32),
                cell_width as u32,
                cell_height as u32,
            );
            canvas
                .fill_rect(r)
                .expect("SDL error: cannot draw rectangle");
        }

        // Draw the things
        for (t, x, y) in &(w.things) {
            let r = Rect::new(
                cell_width * (*x as i32),
                cell_height * (*y as i32),
                cell_width as u32,
                cell_height as u32,
            );
            let c = match t {
                Thing::Food => Color::BLUE,
            };
            canvas.set_draw_color(c);
            canvas
                .fill_rect(r)
                .expect("SDL error: cannot draw rectangle");
        }

        canvas.present();

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
        Some(canvas.window()),
    )
    .expect("Should be able to show messagebox");
}
