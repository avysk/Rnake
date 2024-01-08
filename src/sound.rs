use sdl2::mixer::{Channel, Chunk, Music, DEFAULT_CHANNELS, DEFAULT_FORMAT, DEFAULT_FREQUENCY};
use std::collections::HashMap;

struct NoSounds;

pub struct Sound;

struct Sounds {
    chunks: HashMap<String, Chunk>,
    music: Music<'static>,
}

pub trait Player {
    fn sound_enabled(&self) -> bool;
    fn set_music_volume(&self, level: usize);
    fn play_music(&self);
    fn set_fx_volume(&mut self, level: usize);
}

impl Player for NoSounds {
    fn sound_enabled(&self) -> bool {
        false
    }
    fn set_music_volume(&self, _level: usize) {
        // no sound, do nothing
    }
    fn play_music(&self) {
        // no sound, do nothing
    }
    fn set_fx_volume(&mut self, _level: usize) {
        // no sound, do nothing
    }
}

impl SoundPlayer for NoSounds {}
impl SoundPlayer for Sounds {}

pub trait SoundPlayer: Fx + Player {}

impl Player for Sounds {
    fn sound_enabled(&self) -> bool {
        true
    }
    fn set_music_volume(&self, level: usize) {
        Music::set_volume(16 * level as i32);
    }
    fn play_music(&self) {
        self.music
            .play(-1)
            .expect("Sound is supported, should be able to play music");
    }
    fn set_fx_volume(&mut self, level: usize) {
        for chunk in self.chunks.values_mut() {
            chunk.set_volume(16 * level as i32);
        }
    }
}

macro_rules! with_sounds {
    ($($sound:ident)*) => {
        pub trait Fx {
            $(fn $sound(&self);)*
        }

        impl Fx for NoSounds {
            $(fn $sound(&self) {
                // no sound, do nothing
            })*
        }

        impl Fx for Sounds {
            $(fn $sound(&self) {
                Channel(-1).play(&self.chunks[stringify!($sound)], 0).expect("Should be able to play sound");
            })*
        }

        impl Sound {
            pub fn create() -> Box<dyn SoundPlayer> {
                let mix = sdl2::mixer::init(sdl2::mixer::InitFlag::all());
                if mix.is_err() {
                    return Box::new(NoSounds {});
                }
                let audio = sdl2::mixer::open_audio(
                    DEFAULT_FREQUENCY,
                    DEFAULT_FORMAT,
                    DEFAULT_CHANNELS,
                    512
                );
                if audio.is_err() {
                    return Box::new(NoSounds {});
                }
                sdl2::mixer::allocate_channels(8);
                let music = sdl2::mixer::Music::from_static_bytes(include_bytes!("resources/sounds/kim-lightyear-just-a-dream-wake-up.wav")).expect("Should be able to create music");
                let mut chunks = HashMap::new();
                $(
                let $sound = sdl2::mixer::Chunk::from_raw_buffer(Box::new(*include_bytes!(concat!("resources/sounds/", stringify!($sound), ".wav")))).expect("Should be able to create chunk");
                chunks.insert(stringify!($sound).to_string(), $sound);
                    )*
                Box::new(Sounds {
                    music,
                    chunks
                })
            }
        }
    }
}

with_sounds!(boom food mystery obstacle start wall);
