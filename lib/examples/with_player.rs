use lib_player::{MmlPlayer, MmlPlayerOptions};
use midi_to_mml::{MmlSong, MmlSongOptions};
use std::{path::PathBuf, thread::sleep, time::Duration};

pub fn main() {
    let song = MmlSong::from_path(
        "../assets/FIRE_BIRD_(full_ver_)_(BanG_Dream!_Roselia_9th_Single)_(piano_cover).mid",
        MmlSongOptions::default(),
    )
    .unwrap();
    for track in song.tracks.iter() {
        println!("Track {}", track.name);
        println!("{}\n", track.to_mml());
    }
    let mut player = MmlPlayer::from_song(
        &song,
        MmlPlayerOptions {
            soundfont_path: vec![PathBuf::from(
                "/home/cuikho210/Documents/assets/soundfonts/FluidR3_GM.sf2",
            )],
        },
    )
    .unwrap();
    player.play(None, None).unwrap();
    sleep(Duration::from_secs(120));
}
