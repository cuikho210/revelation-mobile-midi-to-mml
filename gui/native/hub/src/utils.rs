use revelation_mobile_midi_to_mml::Song;
use crate::messages::rust_to_dart::Track;

pub fn get_tracks_from_song(song: &Song) -> Vec<Track> {
    let mut result: Vec<Track> = Vec::new();

    for (index, track) in song.tracks.iter().enumerate() {
        result.push(Track {
            index: index.try_into().unwrap(),
            name: track.name.to_owned(),
            instrument_name: track.instrument.name.to_owned(),
            note_length: track.mml_note_length.try_into().unwrap(),
        });
    }

    result
}
