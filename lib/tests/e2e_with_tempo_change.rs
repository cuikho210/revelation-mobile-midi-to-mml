mod common;

use midi_to_mml::{MmlSong, MmlSongOptions, utils::tick_to_smallest_unit};
use rayon::prelude::*;

use crate::common::TrackTestExt;

#[test]
fn test_e2e() {
    for entry in std::fs::read_dir("../assets").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() && path.extension().map_or(false, |ext| ext == "mid") {
            println!("Testing MIDI file: {:?}", path);
            let options = MmlSongOptions::default();
            let song = MmlSong::from_path(&path, options).unwrap();
            assert_notes_duration(&song);
        }
    }
}

fn assert_notes_duration(song: &MmlSong) {
    song.tracks.par_iter().for_each(|track| {
        let expect_duration = tick_to_smallest_unit(
            track.get_bridge_events_duration(),
            track.ppq,
            track.song_options.smallest_unit,
        );
        let mml_duration = track.get_mml_events_duration();
        assert_eq!(expect_duration, mml_duration);
    });
}
