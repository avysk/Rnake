use crate::widgets::Widget;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct Action {
    content: String,
}

impl Action {
    pub fn new(content: String) -> Self {
        Self { content }
    }
}

impl Widget for Action {
    fn can_activate(&self) -> bool {
        true
    }

    fn present(&self) -> &String {
        &self.content
    }

    fn result(&self) -> usize {
        unimplemented!();
    }

    fn feed(&mut self, event: Event) -> bool {
        matches!(
            event,
            Event::KeyDown {
                keycode: Some(Keycode::Return),
                ..
            } | Event::KeyDown {
                keycode: Some(Keycode::Space),
                ..
            }
        )
    }
}
