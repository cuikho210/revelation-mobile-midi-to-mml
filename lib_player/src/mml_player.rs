use std::{
    path::PathBuf, sync::Arc, thread::{self, JoinHandle}, time::Instant
};
use cpal::Stream;
use revelation_mobile_midi_to_mml::Instrument;
use crate::{Parser, SynthOutputConnection, Synth};

pub struct MmlPlayerOptions {
    pub soundfont_path: PathBuf,
}

pub struct MmlPlayer {
    pub synth: Synth,
    pub stream: Stream,
    pub connection: SynthOutputConnection,
    pub tracks: Vec<Arc<Parser>>,
}

impl MmlPlayer {
    pub fn from_mmls(mmls: Vec<(String, Instrument)>, options: MmlPlayerOptions) -> Self {
        let mut handles: Vec<JoinHandle<Parser>> = Vec::new();
        let mut tracks: Vec<Arc<Parser>> = Vec::new();

        let synth = Synth::new();
        let (stream, connection) = synth.new_stream(options.soundfont_path);

        for mml in mmls {
            let conn = connection.clone();

            let handle = thread::spawn::<_, Parser>(move || {
                Parser::parse(mml.0, mml.1, conn)
            });
            handles.push(handle);
        }

        for handle in handles {
            let parsed = handle.join().unwrap();
            tracks.push(Arc::new(parsed));
        }

        MmlPlayer {
            synth, stream, connection,
            tracks,
        }
    }

    pub fn play(&self) {
        let mut handles: Vec<JoinHandle<()>> = Vec::new();
        let time = Instant::now();
        
        for track in self.tracks.iter() {
            let parsed = track.clone();
            let time = time.to_owned();

            let handle = thread::spawn(move || parsed.play(time));
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }
}
