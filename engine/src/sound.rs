use std::{thread,time};
use kira::{
	manager::{
		AudioManager, AudioManagerSettings,
		backend::DefaultBackend,
	},
	sound::static_sound::{StaticSoundData, StaticSoundSettings},
};

pub fn play_sound() {
    let mut sound_manager =
        AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).unwrap();

    let sound_data =
        StaticSoundData::from_file("content/player_hit.ogg", 
        StaticSoundSettings::default()).unwrap();

    let _ = sound_manager.play(sound_data.clone());
    thread::sleep(time::Duration::from_millis(10));
}

pub fn winner_sound() {
    let mut sound_manager =
        AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).unwrap();

    let sound_data =
        StaticSoundData::from_file("content/winneris.ogg", 
        StaticSoundSettings::default()).unwrap();

    let _ = sound_manager.play(sound_data.clone());
    thread::sleep(time::Duration::from_millis(3500));
}