use sdl2::event::Event;

pub trait Widget {
    fn can_activate(&self) -> bool;
    fn present(&self) -> &String;
    fn result(&self) -> usize;
    // return true from feed() if the parent dialog has to be closed
    fn feed(&mut self, event: Event) -> bool;
}
