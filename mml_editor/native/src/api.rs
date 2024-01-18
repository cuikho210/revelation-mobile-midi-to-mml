use midi_to_mml::{SongOptions, Song};
use midly::{Smf, Track, TrackEventKind, MidiMessage};

pub fn parse_midi(bytes: Vec<u8>, is_auto_split: bool, to_merge: Vec<(usize, usize)>) -> Song {
    let mut result: Vec<String> = Vec::new();

    let song = Song::from_bytes(bytes, SongOptions {
        is_split_track: is_auto_split,
        merge_track: to_merge,
    }).unwrap();
    return song;

    for track in song.tracks.iter() {
        result.push(track.to_mml());
    }

    result
}

pub fn get_track_length(bytes: Vec<u8>) -> usize {
    let smf = Smf::parse(&bytes).unwrap();
    let mut track_length = 0usize;

    for track in smf.tracks {
        let note_length = get_note_length(&track);
        if note_length > 0 {
            track_length += 1;
        }
    }

    track_length
}

fn get_note_length(track: &Track) -> usize {
    let mut note_length = 0usize;

    for event in track {
        match event.kind {
            TrackEventKind::Midi { message, .. } => {
                match message {
                    MidiMessage::NoteOn { .. } => {
                        note_length += 1;
                    },
                    _ => ()
                }
            },
            _ => ()
        }
    }

    note_length
}
