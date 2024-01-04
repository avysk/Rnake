mod sdlwrapper;
mod sound;
mod widgets;
mod world;

use std::process::exit;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::sys::{SDL_Delay, SDL_GetTicks64, Uint32, Uint64};
use serde::{Deserialize, Serialize};

use crate::widgets::{Choice, DialogResult, DialogReturn, Menu, Message, Widget};
use sdlwrapper::SDLWrapper;
use world::{Direction, StepError, StepOk, Thing, World, FIELD_SIZE};

#[derive(Deserialize, Serialize)]
struct RnakeConfig {
    chosen_level: usize,
    last_level: usize,
    speed_index: usize,
}

impl Default for RnakeConfig {
    fn default() -> Self {
        Self {
            chosen_level: 1,
            last_level: 2,
            speed_index: 1,
        }
    }
}

pub fn main() {
    let ttf_context = sdl2::ttf::init().expect("Should be able to construct TTF context");
    let mut sdl = SDLWrapper::new(&FIELD_SIZE, &ttf_context);
    let mut cfg: RnakeConfig =
        confy::load("Rnake", None).expect("There should be no mistakes from confy.");

    sdl.sounds.play_music();
    sdl.sounds.start();
    let mut start_message = Message::new("Press SPACE to start the game.".to_string());
    let mut speed_chooser = Choice::new(
        "Speed".to_string(),
        vec!["slow".to_string(), "normal".to_string(), "fast".to_string()],
        cfg.speed_index,
    );
    let last = cfg.last_level;
    let mut levels = vec![];
    for level in 1..=last {
        levels.push(level.to_string());
    }
    let mut level_chooser = Choice::new("Level".to_string(), levels, cfg.chosen_level - 1);
    let mut menu = Menu::new(vec![
        &mut start_message,
        &mut speed_chooser,
        &mut level_chooser,
    ]);
    let result = menu.run(&mut sdl);
    match result {
        DialogReturn::Result(DialogResult::OK) => {}
        _ => exit(0),
    }

    cfg.speed_index = speed_chooser.result();
    cfg.chosen_level = level_chooser.result() + 1;
    confy::store("Rnake", None, cfg).expect("There should be no confy error when saving config.");
    let play_level = level_chooser.result() + 1;
    let frame_delta = match speed_chooser.result() {
        0 => 180,
        1 => 120,
        2 => 90,
        _ => panic!("Programming error: unknown speed level."),
    };

    'level: loop {
        let mut quit_msg = "You have exited the game.";

        sdl.banner(format!("Level {}", play_level));

        let mut w = World::init(play_level);

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

            // Advance
            match w.step() {
                Err(StepError::Obstacle) => {
                    sdl.sounds.obstacle();
                    quit_msg = "You have hit an obstacle.";
                    break 'running;
                }
                Err(StepError::Wall) => {
                    sdl.sounds.wall();
                    quit_msg = "You have hit the wall.";
                    break 'running;
                }
                Err(StepError::SelfHit) => {
                    sdl.sounds.boom();
                    quit_msg = "You have hit yourself.";
                    break 'running;
                }
                Ok(StepOk::AteFood) => {
                    sdl.sounds.food();
                }
                Ok(StepOk::AteMystery) => {
                    sdl.sounds.mystery();
                }
                Ok(StepOk::Nothing) => {}
            }

            sdl.clear();

            sdl.background();

            // draw field border
            for b in 0..=(FIELD_SIZE + 1) {
                sdl.wall(&0, &b, &0);
                sdl.wall(&0, &b, &(FIELD_SIZE + 1));
                sdl.wall(&0, &0, &b);
                sdl.wall(&0, &(FIELD_SIZE + 1), &b);
            }

            let l = w.snake.len() - 1;
            assert!(
                l >= 2,
                "Programming error: the snake cannot be shorter than 3"
            );

            // draw the snake head
            let head = w
                .snake
                .first()
                .expect("Programming error: a snake cannot be empty");
            if head.dir == head.prev_dir {
                sdl.headstraight(
                    match head.dir {
                        Direction::Down => &0,
                        Direction::Up => &1,
                        Direction::Left => &2,
                        Direction::Right => &3,
                    },
                    &head.coords.x,
                    &head.coords.y,
                );
            } else {
                sdl.headturn(
                    match (&head.dir, &head.prev_dir) {
                        (&Direction::Left, &Direction::Down) => &0,
                        (&Direction::Right, &Direction::Down) => &1,
                        (&Direction::Left, &Direction::Up) => &2,
                        (&Direction::Right, &Direction::Up) => &3,
                        (&Direction::Up, &Direction::Right) => &4,
                        (&Direction::Down, &Direction::Right) => &5,
                        (&Direction::Up, &Direction::Left) => &6,
                        (&Direction::Down, &Direction::Left) => &7,
                        _ => unreachable!("Programming error"),
                    },
                    &head.coords.x,
                    &head.coords.y,
                );
            }

            // draw the body of the snake
            for s in &w.snake[1..l] {
                sdl.body(
                    match (&s.dir, &s.prev_dir, &s.even) {
                        (&Direction::Up, &Direction::Up, &false)
                        | (&Direction::Down, &Direction::Down, &true) => &0,
                        (&Direction::Up, &Direction::Up, &true)
                        | (&Direction::Down, &Direction::Down, &false) => &1,
                        (&Direction::Left, &Direction::Left, &false)
                        | (&Direction::Right, &Direction::Right, &true) => &2,
                        (&Direction::Left, &Direction::Left, &true)
                        | (&Direction::Right, &Direction::Right, &false) => &3,
                        (&Direction::Up, &Direction::Right, _)
                        | (&Direction::Left, &Direction::Down, _) => &4,
                        (&Direction::Up, &Direction::Left, _)
                        | (&Direction::Right, &Direction::Down, _) => &5,
                        (&Direction::Down, &Direction::Right, _)
                        | (&Direction::Left, &Direction::Up, _) => &6,
                        (&Direction::Down, &Direction::Left, _)
                        | (&Direction::Right, &Direction::Up, _) => &7,
                        _ => unreachable!("Programming error"),
                    },
                    &s.coords.x,
                    &s.coords.y,
                );
            }

            // draw the tail of the snake
            let tail = w
                .snake
                .last()
                .expect("Programming error: a snake cannot be empty");
            sdl.tail(
                match tail.dir {
                    Direction::Up => &0,
                    Direction::Down => &1,
                    Direction::Left => &2,
                    Direction::Right => &3,
                },
                &tail.coords.x,
                &tail.coords.y,
            );

            // Draw the things
            for t in &(w.things) {
                match t.what {
                    Thing::Food => {
                        sdl.food(&t.picture_index, &t.x, &t.y);
                    }
                    Thing::Fat => {
                        sdl.fat(&t.picture_index, &t.x, &t.y);
                    }
                    Thing::Lean => {
                        sdl.lean(&t.picture_index, &t.x, &t.y);
                    }
                    Thing::Mystery => {
                        sdl.mystery(&t.picture_index, &t.x, &t.y);
                    }
                    Thing::Obstacle => {
                        sdl.obstacle(&t.picture_index, &t.x, &t.y);
                    }
                    Thing::Wall => {
                        sdl.wall(&t.picture_index, &t.x, &t.y);
                    }
                }
            }

            sdl.score(w.score);
            sdl.present();

            // check if we are at the right moment
            unsafe {
                if SDL_GetTicks64() < next_frame {
                    SDL_Delay((next_frame - SDL_GetTicks64()) as Uint32);
                }
            }

            next_frame = unsafe { SDL_GetTicks64() } + frame_delta;
            turned = false;
        }
        let mut w1 = Message::new(quit_msg.to_string());
        let mut w2 = Message::new("Game over.".to_string());
        let mut w3 = Message::new(format!("Score {}.", w.score));
        let mut w4 = Message::new("Press SPACE to play again,".to_string());
        let mut w5 = Message::new("ESC to exit.".to_string());
        let mut menu = Menu::new(vec![&mut w1, &mut w2, &mut w3, &mut w4, &mut w5]);
        let result = menu.run(&mut sdl);
        match result {
            DialogReturn::Result(DialogResult::OK) => {
                sdl.sounds.start();
                continue 'level;
            }
            _ => {
                break 'level;
            }
        }
    }
}
