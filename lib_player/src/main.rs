use std::{path::PathBuf, time::Instant};
use lib_player::{MmlPlayer, MmlPlayerOptions};
use revelation_mobile_midi_to_mml::{Song, SongOptions};

fn main() {
    test_from_midi();
}

fn test_from_midi() {
    let midi_path = PathBuf::from("./test_resouces/midi/Hitchcock.mid");
    let sf2 = PathBuf::from("./test_resouces/soundfonts/gm.sf2");

    let song = Song::from_path(midi_path, SongOptions {
        auto_boot_velocity: true,
        velocity_min: 8,
        ..Default::default()
    }).unwrap();

    let mmls: Vec<String> = song.tracks.iter().map::<String, _>(|track| track.to_mml()).collect();
    let track_length = mmls.len();

    let time = Instant::now();

    let player = MmlPlayer::from_mmls(mmls, MmlPlayerOptions {
        soundfont_path: sf2,
    });

    println!("Created player with {} tracks in {}ms\n", track_length, time.elapsed().as_millis());

    player.play();
}
