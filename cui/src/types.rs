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
        #[arg(short, long, value_name = "MIDI_PATH")]
        input: String,

        #[arg(short, long, value_name = "OUTPUT_PATH")]
        output: Option<String>
    },

    /// Export MML from MIDI or JSON
    ToMML {
        #[arg(short, long, value_name = "PATH")]
        input: String
    },

    /// List tracks from MIDI or JSON
    ListTracks {
        #[arg(short, long, value_name = "JSON_PATH")]
        input: String
    },

    /// List options from MIDI or JSON
    ListOptions {
        #[arg(short, long, value_name = "JSON_PATH")]
        input: String
    },

    /// Set auto boot velocity of a JSON file
    SetAutoBootVelocity {
        #[arg(short, long, value_name = "JSON_PATH")]
        input: String,

        is_auto_boot_velocity: String
    },

    /// Set velocity min of a JSON file
    SetVelocityMin {
        #[arg(short, long, value_name = "JSON_PATH")]
        input: String,

        value: u8,
    },
    
    /// Set velocity max of a JSON file
    SetVelocityMax {
        #[arg(short, long, value_name = "JSON_PATH")]
        input: String,

        value: u8,
    },

    /// Split track
    Split {
        #[arg(short, long, value_name = "JSON_PATH")]
        input: String,

        index: usize,
    },

    /// Merge tracks
    Merge {
        #[arg(short, long, value_name = "JSON_PATH")]
        input: String,

        index_a: usize,
        index_b: usize,
    },
}
