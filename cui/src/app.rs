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
            Some(Commands::List { input }) => {
                let song = utils::get_song_from_path(input);
                commands::list_tracks(&song);
            }
            Some(Commands::ToMML { input }) => {
                commands::to_mml(input);
            }
            _ => ()
        };
    }
}
