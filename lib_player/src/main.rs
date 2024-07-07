use std::path::PathBuf;
use lib_player::{MmlPlayer, MmlPlayerOptions};
use revelation_mobile_midi_to_mml::{Song, SongOptions};

fn main() {
    test_from_midi();
}

fn test_from_midi() {
    let path = "../test_resouces/midi/Yorushika_-_Rain_with_Cappuccino.mid";
    let midi_path = PathBuf::from(path[1..].to_string());

    // let sf2 = PathBuf::from("./test_resouces/soundfonts/gm.sf2");
    let sf2 = PathBuf::from("/home/cuikho210/Documents/soundfonts/SGM-v2.01-HQ-v3.0.sf2");

    let song = Song::from_path(midi_path, SongOptions {
        auto_boot_velocity: false,
        velocity_min: 8,
        ..Default::default()
    }).unwrap();

    let player = MmlPlayer::from_song(&song, MmlPlayerOptions {
        soundfont_path: sf2,
    });

    player.play();
}
