use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug)]
pub struct PathGroup {
    pub midi_path: PathBuf,
    pub json_path: PathBuf,
}

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Convert a MIDI file to JSON data
    ToJson {
        /// Path to MIDI file
        input: String,
        output: Option<String>,
    },

    ToMML {
        input: String,
    },

    List {
        input: String,
    }
}
