use std::{sync::Arc, thread::{self, JoinHandle}};
use cpal::Stream;
use crate::{Parser, SynthOutputConnection, Synth};

pub struct MmlPlayerOptions {
    pub soundfont_path: String,
}

pub struct MmlPlayer {
    pub synth: Synth,
    pub stream: Stream,
    pub connection: SynthOutputConnection,
    pub tracks: Vec<Arc<Parser>>,
}


impl MmlPlayer {
    pub fn from_mmls(mmls: Vec<String>, options: MmlPlayerOptions) -> Self {
        let mut handles: Vec<JoinHandle<Parser>> = Vec::new();
        let mut tracks: Vec<Arc<Parser>> = Vec::new();

        let synth = Synth::new();
        let (stream, connection) = synth.new_stream(options.soundfont_path);

        for mml in mmls {
            let handle = thread::spawn::<_, Parser>(move || Parser::parse(mml));
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
        
        for (i, track) in self.tracks.iter().enumerate() {
            let conn = self.connection.clone();
            let parsed = track.clone();

            let handle = thread::spawn(move || {
                parsed.play(conn, i as u8)
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }
}
