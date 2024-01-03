use crate::widgets::Widget;
use sdl2::event::Event;

pub struct Message {
    content: String,
}

impl Message {
    pub fn new(content: String) -> Self {
        Self { content }
    }
}

impl Widget for Message {
    fn can_activate(&self) -> bool {
        false
    }

    fn present(&self) -> &String {
        &self.content
    }

    fn result(&self) -> usize {
        unimplemented!();
    }

    fn feed(&mut self, _event: Event) -> bool {
        unimplemented!();
    }
}
