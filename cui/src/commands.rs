use std::path::PathBuf;

use crate::utils;
use revelation_mobile_midi_to_mml::{Song, SongOptions};

pub fn merge_tracks(json_path: &String, index_a: &usize, index_b: &usize) {
    let json_path = PathBuf::from(json_path);
    let mut song = utils::get_song_from_json_path(&json_path).unwrap();
    
    let track_b = song.tracks.get(*index_b).unwrap().to_owned();
    let track_a = song.tracks.get_mut(*index_a).unwrap();
    track_a.merge(&track_b);
    song.tracks.remove(*index_b);

    utils::save_to_json(&song, &json_path);
}

pub fn split_track(json_path: &String, index: &usize) {
    let json_path = PathBuf::from(json_path);
    let mut song = utils::get_song_from_json_path(&json_path).unwrap();

    let (left, right) = song.tracks.get(*index).unwrap().split();
    song.tracks.splice(*index..index+1, [left, right]);

    utils::save_to_json(&song, &json_path);
}

pub fn set_velocity_max(json_path: &String, value: &u8) {
    utils::set_song_options(json_path, |song| {
        println!("velocity_max has been set to {}", value);

        SongOptions {
            velocity_max: *value,
            ..song.options
        }
    });
}

pub fn set_velocity_min(json_path: &String, value: &u8) {
    utils::set_song_options(json_path, |song| {
        println!("velocity_min has been set to {}", value);

        SongOptions {
            velocity_min: *value,
            ..song.options
        }
    });
}

pub fn set_auto_boot_velocity(json_path: &String, is_auto_boot_velocity: bool) {
    utils::set_song_options(json_path, |song| {
        println!("auto_boot_velocity has been set to {}", is_auto_boot_velocity);

        SongOptions {
            auto_boot_velocity: is_auto_boot_velocity,
            ..song.options
        }
    });
}

pub fn midi_to_json(input: &String, output: &Option<String>) {
    let path_group = utils::to_path_group(input, output);
    let song = Song::from_path(
        path_group.midi_path.to_owned(),
        SongOptions::default(),
    ).unwrap();

    utils::save_to_json(&song, &path_group.json_path);
}

pub fn list_tracks(song: &Song) {
    for (index, track) in song.tracks.iter().enumerate() {
        utils::print_track_title(&index, track);
    }
}

pub fn list_options(song: &Song) {
    println!("{:#?}", song.options);
}

pub fn to_mml(input: &String) {
    let song = utils::get_song_from_path(input);

    println!("\n------------------------------------------------------------------------------------");
    println!("|     MIDI to MML - https://github.com/cuikho210/revelation-mobile-midi-to-mml     |");
    println!("------------------------------------------------------------------------------------\n");

    for (index, track) in song.tracks.iter().enumerate() {
        utils::print_track_title(&index, track);
        utils::print_track_mml(track);
    }
}
