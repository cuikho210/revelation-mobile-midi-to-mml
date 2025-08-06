use midi_to_mml::{MmlEvent, MmlSong, MmlSongOptions, utils::compute_position_in_smallest_unit};
use rayon::prelude::*;

const MIDI_FILE_PATH: &str = "../assets/cloudless-yorushika.mid";

#[test]
fn test_e2e() {
    let options = MmlSongOptions::default();
    let song = MmlSong::from_path(MIDI_FILE_PATH, options).unwrap();
    assert_only_1_tempo_change(&song);
    assert_notes(&song);
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
