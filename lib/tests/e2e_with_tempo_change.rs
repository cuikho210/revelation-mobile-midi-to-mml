mod common;

use midi_to_mml::{
    MmlEvent, MmlSong, MmlSongOptions,
    utils::{compute_position_in_smallest_unit, tick_to_smallest_unit},
};

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
            assert_tempo_position(&song);
        }
    }
}

fn assert_tempo_position(song: &MmlSong) {
    let track_tempos: Vec<Vec<(usize, usize)>> = song
        .tracks
        .iter()
        .map(|track| {
            track
                .events
                .iter()
                .enumerate()
                .filter_map(|(i, e)| {
                    if let MmlEvent::Tempo(_, pos) = e {
                        Some((*pos, compute_position_in_smallest_unit(&track.events, i)))
                    } else {
                        None
                    }
                })
                .collect()
        })
        .collect();

    for track in track_tempos.iter() {
        for (i, (expect, actual)) in track.iter().enumerate() {
            println!("Assert tempo at {i}");
            assert_eq!(expect, actual);
        }
    }
}

fn assert_notes_duration(song: &MmlSong) {
    song.tracks.iter().for_each(|track| {
        let expect_duration = tick_to_smallest_unit(
            track.get_bridge_events_duration(),
            track.ppq,
            track.song_options.smallest_unit,
        );
        let mml_duration = track.get_mml_events_duration();
        assert_eq!(expect_duration, mml_duration);
    });
}
