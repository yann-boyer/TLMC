extern crate sdl2;

use sdl2::mixer::{InitFlag, AUDIO_S16LSB, DEFAULT_CHANNELS, Music};

pub struct SoundSystem<'a> {
    beep_sound: Option<Music<'a>>
}

impl <'a> SoundSystem<'a> {
    pub fn new() -> SoundSystem<'a> {
        let mut sound_system = SoundSystem {
            beep_sound: None
        };

        sdl2::mixer::open_audio(44100, AUDIO_S16LSB, DEFAULT_CHANNELS, 1024).expect("Unable to open sdl2 mixer audio");

        sdl2::mixer::init(InitFlag::MP3).expect("Unable to init SDL mixer !");

        sdl2::mixer::allocate_channels(4);

        sound_system.beep_sound = Some(Music::from_file("chip8_beep.mp3").expect("Unable to load beep sound !"));

        Music::set_volume(50);

        sound_system
    }

    pub fn play_beep_sound(&self) {
        self.beep_sound.as_ref().unwrap().play(0).unwrap();
    }
}
