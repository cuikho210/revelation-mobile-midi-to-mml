use crate::{
    types::{Commands, Cli},
    commands, utils,
};
use clap::Parser;

pub struct App;
impl App {
    pub fn new() {
        let cli = Cli::parse();

        match &cli.command {
            Some(Commands::ToJson { input, output }) => {
                let path_group = utils::to_path_group(input, output);
                let song = utils::get_song_from_midi_path(&path_group.midi_path).unwrap();
                let json = commands::to_json(&song);

                utils::save_json(&json, &path_group.json_path);
            }
            Some(Commands::ToMML { input }) => {
                let song = utils::get_song_from_path(input);
                println!("{}", commands::to_mml(&song));
            }
            Some(Commands::ListTracks { input }) => {
                let song = utils::get_song_from_path(input);
                println!("{}",commands::list_tracks(&song));
            }
            Some(Commands::ListOptions { input }) => {
                let song = utils::get_song_from_path(input);
                println!("{}", commands::list_options(&song));
            }
            Some(Commands::SetAutoBootVelocity { input, is_auto_boot_velocity }) => {
                utils::modify_json_file(input, |song| {
                    commands::set_auto_boot_velocity(
                        song,
                        utils::string_to_bool_arg(is_auto_boot_velocity),
                    );
                    println!("Options has been set to {:#?}", song.options);
                });
            }
            Some(Commands::SetVelocityMin { input, value }) => {
                utils::modify_json_file(input, |song| {
                    commands::set_velocity_min(song, value);
                    println!("Options has been set to {:#?}", song.options);
                });
            }
            Some(Commands::SetVelocityMax { input, value }) => {
                utils::modify_json_file(input, |song| {
                    commands::set_velocity_max(song, value);
                    println!("Options has been set to {:#?}", song.options);
                });
            }
            Some(Commands::Split { input, index }) => {
                utils::modify_json_file(input, |song| {
                    commands::split_track(song, index);
                });
            }
            Some(Commands::Merge { input, index_a, index_b }) => {
                utils::modify_json_file(input, |song| {
                    commands::merge_tracks(song, index_a, index_b);
                });
            }
            _ => ()
        };
    }
}
