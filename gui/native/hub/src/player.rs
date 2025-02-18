use crate::{
    logger::{log, Logger},
    messages::{
        rust_to_dart::{SignalMmlNoteOn, SignalOnTrackEnd},
        types::SignalPlayStatus,
    },
};
use anyhow::Result;
use lib_player::{MmlPlayer, MmlPlayerOptions, NoteOnCallbackData};
use revelation_mobile_midi_to_mml::Instrument;
use std::sync::Arc;
use tokio::{sync::Mutex, task};

pub struct PlayerState {
    pub player: MmlPlayer,
    pub playback_state: SignalPlayStatus,
}

impl PlayerState {
    pub fn new() -> Result<Self> {
        Ok(PlayerState {
            player: MmlPlayer::new(MmlPlayerOptions {
                soundfont_path: Vec::new(),
            })?,
            playback_state: SignalPlayStatus::Stop,
        })
    }

    pub fn parse_mmls(&mut self, mmls: Vec<(String, Instrument)>) -> Result<()> {
        let _ = self.player.parse_mmls(mmls);
        Ok(())
    }

    pub fn play(&mut self) -> Result<()> {
        let note_on_callback: Arc<fn(NoteOnCallbackData)> = Arc::new(|data: NoteOnCallbackData| {
            SignalMmlNoteOn {
                track_index: data.track_index as u64,
                char_index: data.char_index as u64,
                char_length: data.char_length as u64,
            }
            .send_signal_to_dart();
        });

        let track_end_callback: Arc<fn(usize)> = Arc::new(|track_index| {
            SignalOnTrackEnd {
                track_index: track_index as u64,
            }
            .send_signal_to_dart();
        });

        self.playback_state = SignalPlayStatus::Play;
        let _ = self
            .player
            .play(Some(note_on_callback), Some(track_end_callback));
        Ok(())
    }

    pub fn pause(&mut self) -> Result<()> {
        let _ = self.player.pause();
        self.playback_state = SignalPlayStatus::Pause;
        Ok(())
    }

    pub fn stop(&mut self) -> Result<()> {
        self.player.stop()?;
        self.playback_state = SignalPlayStatus::Stop;
        Ok(())
    }

    pub fn load_soundfont_from_bytes(&mut self, bytes: Vec<u8>) -> Result<()> {
        self.player.load_soundfont_from_bytes(bytes)?;
        Ok(())
    }

    pub fn load_soundfont_from_bytes_parallel(&mut self, list_bytes: Vec<Vec<u8>>) -> Result<()> {
        self.player.load_soundfont_from_bytes_parallel(list_bytes)
    }
}

pub fn parse_mmls_parallel(
    player_state: Arc<Mutex<PlayerState>>,
    logger_state: Arc<Mutex<Logger>>,
    mmls: Vec<(String, Instrument)>,
) {
    task::spawn(async move {
        log(logger_state.clone(), crate::logger::LogType::ParseMmlInit).await;

        let mut player = player_state.lock().await;
        if let Err(err) = player.parse_mmls(mmls) {
            log(
                logger_state.clone(),
                crate::logger::LogType::Error(err.to_string()),
            )
            .await;
        }

        log(logger_state, crate::logger::LogType::ParseMmlEnd).await;
    });
}
