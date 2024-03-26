use crate::utils;
use revelation_mobile_midi_to_mml::{Song, SongOptions};

pub fn merge_tracks(song: &mut Song, index_a: &usize, index_b: &usize) {
    let track_b = song.tracks.get(*index_b).unwrap().to_owned();
    let track_a = song.tracks.get_mut(*index_a).unwrap();
    track_a.merge(&track_b);
    song.tracks.remove(*index_b);
}

pub fn split_track(song: &mut Song, index: &usize) {
    let (left, right) = song.tracks.get(*index).unwrap().split();
    song.tracks.splice(*index..index+1, [left, right]);
}

pub fn set_velocity_max(song: &mut Song, value: &u8) {
    let options = SongOptions {
        velocity_max: *value,
        ..song.options
    };

    song.set_song_options(options);
}

pub fn set_velocity_min(song: &mut Song, value: &u8) {
    let options = SongOptions {
        velocity_min: *value,
        ..song.options
    };

    song.set_song_options(options);
}

pub fn set_auto_boot_velocity(song: &mut Song, auto_boot_velocity: bool) {
    let options = SongOptions {
        auto_boot_velocity,
        ..song.options
    };

    song.set_song_options(options);
}

pub fn to_json(song: &Song) -> String {
    serde_json::to_string(song).unwrap()
}

pub fn list_tracks(song: &Song) -> String {
    let mut result = String::new();

    for (index, track) in song.tracks.iter().enumerate() {
        result.push_str(&utils::get_track_title(&index, track));
    }

    result
}

pub fn list_options(song: &Song) -> String {
    format!("{:#?}", song.options)
}

pub fn to_mml(song: &Song) -> String {
    let mut result = "\n".to_string();
    result.push_str("------------------------------------------------------------------------------------\n");
    result.push_str("|     MIDI to MML - https://github.com/cuikho210/revelation-mobile-midi-to-mml     |\n");
    result.push_str("------------------------------------------------------------------------------------\n\n");

    for (index, track) in song.tracks.iter().enumerate() {
        result.push_str(&utils::get_track_title(&index, track));
        result.push('\n');
        result.push_str(&utils::get_track_mml(track));
        result.push('\n');
    }

    result
}
