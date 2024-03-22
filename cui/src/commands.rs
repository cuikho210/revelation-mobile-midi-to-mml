use crate::utils;
use revelation_mobile_midi_to_mml::{Song, SongOptions};

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
    for track in song.tracks.iter() {
        utils::print_track_title(track);
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

    for track in song.tracks.iter() {
        utils::print_track_title(track);
        utils::print_track_mml(track);
    }
}
