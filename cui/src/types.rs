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
    ToJson { input: String, output: Option<String> },

    /// Export MML from MIDI or JSON
    ToMML { input: String },

    /// List tracks from MIDI or JSON
    ListTracks { input: String },

    /// List options from MIDI or JSON
    ListOptions { input: String },

    /// Set auto boot velocity of a JSON file
    SetAutoBootVelocity { input: String, is_auto_boot_velocity: String },

    /// Set velocity min of a JSON file
    SetVelocityMin { input: String, value: u8 },
    
    /// Set velocity max of a JSON file
    SetVelocityMax { input: String, value: u8 },

    /// Split track
    Split { input: String, index: usize },
}
