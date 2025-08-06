use midi_to_mml::{MmlEvent, MmlSong, MmlSongOptions, utils::compute_position_in_smallest_unit};
use rayon::prelude::*;
use tracing::debug;

#[test]
fn test_e2e() {
    for entry in std::fs::read_dir("../assets").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() && path.extension().map_or(false, |ext| ext == "mid") {
            debug!("Testing MIDI file: {:?}", path);
            let options = MmlSongOptions::default();
            let song = MmlSong::from_path(&path, options).unwrap();
            assert_tempo_position(&song);
            assert_notes(&song);
        }
    }
}

fn assert_tempo_position(song: &MmlSong) {
    let track_tempos: Vec<Vec<(usize, usize)>> = song
        .tracks
        .par_iter()
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
            debug!("Assert tempo at {i}");
            assert_eq!(expect, actual);
        }
    }
}

fn assert_notes(song: &MmlSong) {
    song.tracks.par_iter().for_each(|track| {
        for (i, e) in track.events.iter().enumerate() {
            if let MmlEvent::Note(note) = e
                && !note.is_part_of_chord
            {
                assert_eq!(
                    note.position_in_smallest_unit,
                    compute_position_in_smallest_unit(&track.events, i)
                );
            }
        }
    });
}
