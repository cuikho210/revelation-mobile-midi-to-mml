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
                commands::midi_to_json(input, output);
            }
            Some(Commands::ToMML { input }) => {
                commands::to_mml(input);
            }
            Some(Commands::ListTracks { input }) => {
                let song = utils::get_song_from_path(input);
                commands::list_tracks(&song);
            }
            Some(Commands::ListOptions { input }) => {
                let song = utils::get_song_from_path(input);
                commands::list_options(&song);
            }
            Some(Commands::SetAutoBootVelocity { input, is_auto_boot_velocity }) => {
                commands::set_auto_boot_velocity(
                    input,
                    utils::string_to_bool_arg(is_auto_boot_velocity),
                );
            }
            Some(Commands::SetVelocityMin { input, value }) => {
                commands::set_velocity_min(input, value);
            }
            Some(Commands::SetVelocityMax { input, value }) => {
                commands::set_velocity_max(input, value);
            }
            Some(Commands::Split { input, index }) => {
                commands::split_track(input, index);
            }
            Some(Commands::Merge { input, index_a, index_b }) => {
                commands::merge_tracks(input, index_a, index_b);
            }
            _ => ()
        };
    }
}
