mod common;

use midi_to_mml::{MmlEvent, MmlSong, MmlSongOptions, utils::tick_to_smallest_unit};
use rayon::prelude::*;

use crate::common::TrackTestExt;

const MIDI_FILE_PATH: &str = "../assets/cloudless-yorushika.mid";

#[test]
fn test_e2e() {
    let options = MmlSongOptions::default();
    let song = MmlSong::from_path(MIDI_FILE_PATH, options).unwrap();
    assert_only_1_tempo_change(&song);
    assert_notes_duration(&song);
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

fn assert_only_1_tempo_change(song: &MmlSong) {
    song.tracks.par_iter().for_each(|t| {
        let count_tempo_events = t
            .events
            .iter()
            .filter(|e| {
                if let MmlEvent::Tempo(_, _) = e {
                    true
                } else {
                    false
                }
            })
            .count();
        assert_eq!(count_tempo_events, 1);
    });
}
