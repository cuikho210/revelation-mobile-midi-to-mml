use std::{path::PathBuf, thread::sleep, time::Duration};

use lib_player::{MmlPlayer, MmlPlayerOptions};
use midi_to_mml::{MmlSong, MmlSongOptions};

pub fn main() {
    let song = MmlSong::from_path(
        "../assets/FIRE_BIRD_(full_ver_)_(BanG_Dream!_Roselia_9th_Single)_(piano_cover).mid",
        MmlSongOptions::default(),
    )
    .unwrap();
    let mut player = MmlPlayer::from_song(
        &song,
        MmlPlayerOptions {
            soundfont_path: vec![PathBuf::from(
                "/home/cuikho210/Documents/assets/soundfonts/FluidR3_GM.sf2",
            )],
        },
    );
    player.play(None, None);
    sleep(Duration::from_secs(120));
}
