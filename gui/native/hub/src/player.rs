use lib_player::{MmlPlayer, MmlPlayerOptions, NoteOnCallbackData};
use revelation_mobile_midi_to_mml::Instrument;
use rinf::debug_print;
use tokio::{task, sync::Mutex};
use std::{sync::Arc, time::Instant};
use crate::messages::{rust_to_dart::SignalMmlNoteOn, types::SignalPlayStatus};

pub struct PlayerState {
    pub player: MmlPlayer,
    pub playback_state: SignalPlayStatus,
} 

impl PlayerState {
    pub fn new() -> Self {
        PlayerState {
            player: MmlPlayer::new(MmlPlayerOptions {
                soundfont_path: Vec::new(),
            }),
            playback_state: SignalPlayStatus::Stop,
        }
    }

    pub fn parse_mmls(&mut self, mmls: Vec<(String, Instrument)>) {
        self.player.parse_mmls(mmls);
    }

    pub fn play(&mut self) {
        let callback: Arc<fn(NoteOnCallbackData)> = Arc::new(|data: NoteOnCallbackData| {
            SignalMmlNoteOn {
                track_id: data.track_index as u64,
                char_index: data.char_index as u64,
                char_length: data.char_length as u64,
            }.send_signal_to_dart();
        });

        self.player.play(Some(callback));
        self.playback_state = SignalPlayStatus::Play;
        debug_print!("Set player play");
    }

    pub fn pause(&mut self) {
        self.player.pause();
        self.playback_state = SignalPlayStatus::Pause;
        debug_print!("Set player pause");
    }

    pub fn stop(&mut self) {
        self.player.stop();
        self.playback_state = SignalPlayStatus::Stop;
        debug_print!("Set player stop");
    }

    pub fn load_soundfont_from_bytes(&mut self, bytes: Vec<u8>) -> Result<(), String> {
        let time = Instant::now();
        self.player.load_soundfont_from_bytes(bytes)?;

        let elapsed = time.elapsed();
        debug_print!("[load_soundfont_from_bytes] loaded a soundfont in {} ms", elapsed.as_millis());

        Ok(())
    }

    pub fn load_soundfont_from_bytes_parallel(&mut self, list_bytes: Vec<Vec<u8>>) -> Result<(), String> {
        let time = Instant::now();
        let length = list_bytes.len();
        self.player.load_soundfont_from_bytes_parallel(list_bytes)?;

        let elapsed = time.elapsed();
        debug_print!(
            "[load_soundfont_from_bytes_parallel] loaded {} soundfonts in {} ms",
            length,
            elapsed.as_millis()
        );

        Ok(())
    }
}

pub fn parse_mmls_parallel(
    player_state: Arc<Mutex<PlayerState>>,
    mmls: Vec<(String, Instrument)>,
) {
    task::spawn(async move {
        let mut player = player_state.lock().await;
        player.parse_mmls(mmls);
    });
}

