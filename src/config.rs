use crate::sdlwrapper::SDLWrapper;
use crate::widgets::{Action, Choice, DialogResult, DialogReturn, Menu, Message, Widget};
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct RnakeConfig {
    pub chosen_level: usize,
    last_level: usize,
    pub speed_index: usize,
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

pub fn config_dialog(cfg: &mut RnakeConfig, sdl: &mut SDLWrapper) {
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
    let mut save = Action::new("Save settings".to_string());
    let mut cancel = Action::new("Discard changes".to_string());
    let mut esc_message = Message::new("ESC to discard changes".to_string());
    let mut menu = Menu::new(vec![
        &mut speed_chooser,
        &mut level_chooser,
        &mut save,
        &mut cancel,
        &mut esc_message,
    ]);

    match menu.run(sdl) {
        DialogReturn::Index(2) => {
            cfg.speed_index = speed_chooser.result();
            cfg.chosen_level = level_chooser.result() + 1;
        }
        DialogReturn::Index(3) | DialogReturn::Result(DialogResult::Cancel) => {}
        _ => panic!("Unknown result from options dialogue"),
    }
}
