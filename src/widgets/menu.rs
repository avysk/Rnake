use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::sdlwrapper::SDLWrapper;
use crate::widgets::Widget;

#[derive(Clone, Copy)]
pub enum DialogResult {
    OK,
    CANCEL,
}

pub enum DialogReturn {
    Result(DialogResult),
    Index(usize),
}

pub struct Menu<'a> {
    widgets: Vec<&'a mut dyn Widget>,
}

impl<'a> Menu<'a> {
    pub fn new(widgets: Vec<&'a mut dyn Widget>) -> Self {
        Self { widgets }
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
                if total > 0 {
                    if w.can_activate() {
                        if activated == current {
                            repr = "- ".to_owned() + s + " -";
                            activated_index = Some(idx);
                        }
                        current += 1;
                    }
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
                        activated += 1;
                        activated %= total;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Up),
                        ..
                    } if total > 0 => {
                        activated += total - 1;
                        activated %= total;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Space),
                        ..
                    } => {
                        return DialogReturn::Result(DialogResult::OK);
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        return DialogReturn::Result(DialogResult::CANCEL);
                    }
                    _ if total > 0 => {
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
