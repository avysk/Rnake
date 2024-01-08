use crate::widgets::Widget;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct Choice<'a> {
    name: String,
    content: &'a Vec<String>,
    chosen: usize,
    presentation: String,
}

impl<'a> Choice<'a> {
    pub fn new(name: String, content: &'a Vec<String>, chosen: usize) -> Self {
        let name1 = name.clone();
        Self {
            name,
            content,
            chosen,
            presentation: format!("{} > {} <", name1, content[chosen]),
        }
    }
    fn update_presentation(&mut self) {
        self.presentation = format!("{} > {} <", self.name, self.content[self.chosen]);
    }
}

impl<'a> Widget for Choice<'a> {
    fn can_activate(&self) -> bool {
        true
    }

    fn present(&self) -> &String {
        &self.presentation
    }

    fn result(&self) -> usize {
        self.chosen
    }

    fn feed(&mut self, event: Event) -> bool {
        let total = self.content.len();
        match event {
            Event::KeyDown {
                keycode: Some(Keycode::Left),
                ..
            } => {
                self.chosen += total - 1;
                self.chosen %= total;
                self.update_presentation();
            }
            Event::KeyDown {
                keycode: Some(Keycode::Right),
                ..
            } => {
                self.chosen += 1;
                self.chosen %= total;
                self.update_presentation();
            }
            _ => {}
        }
        false
    }
}
