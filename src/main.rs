mod sdlwrapper;
mod sound;
mod world;

use std::cmp::min;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::sys::{SDL_Delay, SDL_GetTicks64, Uint32, Uint64};

use sdlwrapper::SDLWrapper;
use world::{StepError, StepOk, Thing, World, FIELD_SIZE};

const FRAME_DELTA: Uint64 = 60;
// update screen after the given number of SDL ticks
const WAIT: Uint64 = 20;

pub fn main() {
    let ttf_context = sdl2::ttf::init().expect("Should be able to construct TTF context");
    let mut sdl = SDLWrapper::new(&FIELD_SIZE, &ttf_context);

    sdl.sounds.start();
    sdl.messages(vec!["Press SPACE to start the game"]);
    'waiting_start: loop {
        for event in sdl.events.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    break 'waiting_start;
                }
                _ => unsafe {
                    SDL_Delay(100);
                },
            }
        }
    }

    'game: loop {
        let mut w = World::init();

        let mut next_frame: Uint64 = 0;
        let mut turned = false;

        'running: loop {
            // process quit and turn the snake events
            for event in sdl.events.poll_iter() {
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
                    sdl.sounds.wall();
                    break 'running;
                }
                Err(StepError::SelfHit) => {
                    sdl.sounds.boom();
                    break 'running;
                }
                Ok(StepOk::AteFood) => {
                    sdl.sounds.food();
                }
                Ok(StepOk::Nothing) => {}
            }

            sdl.clear();

            // draw field border
            for b in 0..=(FIELD_SIZE + 1) {
                let wall = &Color::YELLOW;
                sdl.rect(&b, &0, wall);
                sdl.rect(&b, &(FIELD_SIZE + 1), wall);
                sdl.rect(&0, &b, wall);
                sdl.rect(&(FIELD_SIZE + 1), &b, wall);
            }

            // draw the snake head
            let (hx, hy) = w
                .snake
                .first()
                .expect("Programming error: a snake cannot be empty");
            sdl.rect(&(*hx + 1), &(*hy + 1), &Color::GREEN);

            // draw rest of the snake
            for (bx, by) in &w.snake[1..] {
                sdl.rect(&(*bx + 1), &(*by + 1), &Color::GRAY);
            }

            // Draw the things
            for (t, x, y) in &(w.things) {
                let c = match t {
                    Thing::Food => Color::BLUE,
                };
                sdl.rect(&(*x + 1), &(*y + 1), &c);
            }

            sdl.present();

            next_frame = unsafe { SDL_GetTicks64() } + FRAME_DELTA;
            turned = false;

            if w.things.is_empty() {
                w.add_thing();
            }
        }
        sdl.messages(vec![
            "Game over.",
            format!("Score {}.", w.score).as_ref(),
            "Press SPACE to play again,",
            "ESC to exit.",
        ]);
        loop {
            for event in sdl.events.poll_iter() {
                match event {
                    Event::KeyDown {
                        keycode: Some(Keycode::Space),
                        ..
                    } => {
                        sdl.sounds.start();
                        continue 'game;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        break 'game;
                    }
                    _ => unsafe {
                        SDL_Delay(100);
                    },
                }
            }
        }
    }
}
