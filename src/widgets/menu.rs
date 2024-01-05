use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mixer::Channel;

use crate::sdlwrapper::SDLWrapper;
use crate::widgets::Widget;

#[derive(Clone, Copy)]
pub enum DialogResult {
    Ok,
    Cancel,
}

pub enum DialogReturn {
    Result(DialogResult),
    Index(usize),
}

pub struct Menu<'a> {
    click: Result<sdl2::mixer::Chunk, String>,
    widgets: Vec<&'a mut dyn Widget>,
}

impl<'a> Menu<'a> {
    pub fn new(widgets: Vec<&'a mut dyn Widget>) -> Self {
        let click = sdl2::mixer::Chunk::from_raw_buffer(Box::new(*include_bytes!(
            "../resources/sounds/click.wav"
        )));
        Self { click, widgets }
    }

    pub fn click(&self) {
        if let Ok(chunk) = &self.click {
            Channel(1)
                .play(chunk, 0)
                .expect("Should be able to play click.");
        }
    }

    // Return the index of the widget which closed the menu
    pub fn run(&mut self, sdl: &mut SDLWrapper) -> DialogReturn {
        let total = self.widgets.iter().filter(|w| w.can_activate()).count();
        let mut activated = 0;
        loop {
            let mut messages = vec![];
            let mut current = 0;
            let mut activated_index = None;
            for (idx, w) in self.widgets.iter().enumerate() {
                let s = w.present();
                let mut repr = s.to_string();
                if total > 0 && w.can_activate() {
                    if activated == current {
                        repr = "- ".to_owned() + s + " -";
                        activated_index = Some(idx);
                    }
                    current += 1;
                }
                messages.push(repr);
            }
            sdl.messages(&messages);
            for event in sdl.events.poll_iter() {
                match event {
                    Event::KeyDown {
                        keycode: Some(Keycode::Down),
                        ..
                    } if total > 0 => {
                        self.click();
                        activated += 1;
                        activated %= total;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Up),
                        ..
                    } if total > 0 => {
                        self.click();
                        activated += total - 1;
                        activated %= total;
                    }
                    // SPACE works only if there is no widgets which can be activated
                    Event::KeyDown {
                        keycode: Some(Keycode::Space),
                        ..
                    } if total == 0 => {
                        self.click();
                        return DialogReturn::Result(DialogResult::Ok);
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        self.click();
                        return DialogReturn::Result(DialogResult::Cancel);
                    }
                    Event::KeyDown { .. } if total > 0 => {
                        self.click();
                        let number = activated_index
                            .expect("Programming error: there should be an activated widget.");
                        let result = self.widgets[number].feed(event);
                        if result {
                            return DialogReturn::Index(number);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
