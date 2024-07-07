use std::{path::PathBuf, time::Instant};
use lib_player::{MmlPlayer, MmlPlayerOptions};
use revelation_mobile_midi_to_mml::{Instrument, Song, SongOptions};

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

    let mut char_length = 0usize;

    let mmls: Vec<(String, Instrument)> = song.tracks.iter().map::<(String, Instrument), _>(|track| {
        let mml = track.to_mml();
        char_length += mml.len();

        (mml, track.instrument.to_owned())
    }).collect();

    let track_length = mmls.len();
    let time = Instant::now();

    let player = MmlPlayer::from_mmls(mmls, MmlPlayerOptions {
        soundfont_path: sf2,
    });

    println!("Created player with {} tracks and {} chars in {}ms\n", track_length, char_length, time.elapsed().as_millis());

    player.play();
}
