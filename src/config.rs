use serde::{Deserialize, Serialize};

use crate::sdlwrapper::SDLWrapper;
use crate::widgets::{
    Action, Choice, DialogResult, DialogReturn, Menu, Message, Widget, MENU_SOUND_LEVEL,
};

const RNAKE_CONFIG_VERSION: u8 = 2;

#[derive(Debug, Default, Deserialize, Serialize)]
struct RnakeConfigVersion {
    version: u8,
}

#[derive(Clone, Deserialize, Serialize)]
struct RnakeConfigV1 {
    chosen_level: usize,
    last_level: usize,
    speed_index: usize,
    version: u8,
}

impl Default for RnakeConfigV1 {
    fn default() -> Self {
        Self {
            chosen_level: 1,
            last_level: 1,
            speed_index: 1,
            version: 1,
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct RnakeConfigV2 {
    pub chosen_level: usize,
    last_level: usize,
    pub speed_index: usize,
    music_volume: usize,
    menu_volume: usize,
    fx_volume: usize,
    version: u8,
}

impl Default for RnakeConfigV2 {
    fn default() -> Self {
        Self {
            chosen_level: 1,
            last_level: 1,
            speed_index: 1,
            music_volume: 4,
            menu_volume: 6,
            fx_volume: 6,
            version: RNAKE_CONFIG_VERSION,
        }
    }
}
pub type Configuration = RnakeConfigV2;

impl Configuration {
    pub fn new() -> Self {
        let v: RnakeConfigVersion =
            confy::load("Rnake", None).expect("Should be able to read version from configuration");
        if v.version == RNAKE_CONFIG_VERSION {
            // We have a config of a proper version
            confy::load("Rnake", None).expect("Should be able to read config file")
        } else {
            let mut cfg: RnakeConfigV2 = RnakeConfigV2::default();
            match v.version {
                0 => {}
                1 => {
                    let cfg1: RnakeConfigV1 = confy::load("Rnake", None)
                        .expect("Should be able to read config version 1");
                    cfg.speed_index = cfg1.speed_index;
                    cfg.last_level = cfg1.last_level;
                    cfg.chosen_level = cfg1.chosen_level;
                }
                n => {
                    panic!("Programming error: unknown config version {}", n);
                }
            }
            confy::store("Rnake", None, cfg.clone()).expect("Should be able to store config");
            cfg
        }
    }
    pub fn config_dialog(&mut self, sdl: &mut SDLWrapper) {
        let items = &vec!["slow".to_string(), "normal".to_string(), "fast".to_string()];
        let mut speed_chooser = Choice::new("Speed".to_string(), items, self.speed_index);
        let last = self.last_level;
        let levels = (1..=last).map(|num| num.to_string()).collect();
        let mut level_chooser = Choice::new("Level".to_string(), &levels, self.chosen_level - 1);
        let mut menu_items: Vec<&mut dyn Widget> = vec![&mut speed_chooser, &mut level_chooser];

        // Jumping through some hoops to prevent Rust from complaining about uninitialized variables
        let empty = &vec![String::new()];
        let mut music_volume_chooser = Choice::new("Fake".to_string(), empty, 0);
        let mut menu_volume_chooser = Choice::new("Fake".to_string(), empty, 0);
        let mut fx_volume_chooser = Choice::new("Fake".to_string(), empty, 0);
        let sound_volumes: Vec<String>;
        if sdl.sounds.sound_enabled() {
            sound_volumes = (0..=8).map(|num| num.to_string()).collect();
            music_volume_chooser =
                Choice::new("Music level".to_string(), &sound_volumes, self.music_volume);
            menu_volume_chooser = Choice::new(
                "Menu sounds level".to_string(),
                &sound_volumes,
                self.menu_volume,
            );
            fx_volume_chooser = Choice::new(
                "Game sounds level".to_string(),
                &sound_volumes,
                self.fx_volume,
            );
            menu_items.push(&mut music_volume_chooser);
            menu_items.push(&mut menu_volume_chooser);
            menu_items.push(&mut fx_volume_chooser);
        }
        let mut save = Action::new("Save settings".to_string());
        let mut cancel = Action::new("Discard changes".to_string());
        let mut esc_message = Message::new("ESC to discard changes".to_string());
        let save_index = menu_items.len();
        menu_items.push(&mut save);
        menu_items.push(&mut cancel);
        menu_items.push(&mut esc_message);
        let mut menu = Menu::new(menu_items);

        match menu.run(sdl) {
            DialogReturn::Index(n) if n == save_index => {
                self.speed_index = speed_chooser.result();
                self.chosen_level = level_chooser.result() + 1;
                if sdl.sounds.sound_enabled() {
                    self.music_volume = music_volume_chooser.result();
                    sdl2::mixer::Music::set_volume(16 * self.music_volume as i32);
                    self.menu_volume = menu_volume_chooser.result();
                    unsafe {
                        MENU_SOUND_LEVEL = self.menu_volume;
                    }
                    self.fx_volume = fx_volume_chooser.result();
                    sdl.sounds.set_fx_volume(self.fx_volume);
                    confy::store("Rnake", None, self)
                        .expect("Should be no confy error when saving configuration");
                }
            }
            DialogReturn::Index(n) if n == save_index + 1 => {}
            DialogReturn::Result(DialogResult::Cancel) => {}
            _ => panic!("Unknown result from options dialogue"),
        }
    }
}
