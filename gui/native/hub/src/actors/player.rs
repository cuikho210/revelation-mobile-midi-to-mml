use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use lib_player::{MmlPlayer, MmlPlayerOptions, NoteOnCallbackData};
use messages::{
    actor::Actor,
    prelude::{Address, Context, Handler},
};
use midi_to_mml::MmlTrack;
use rinf::RustSignal;
use tokio::{spawn, sync::Mutex, task::JoinSet};
use tracing::info;

use crate::{
    actors::common::{ActorName, ListenDartSignal},
    signals::{
        SignalLoadSoundfontRequest, SignalMmlNoteOn, SignalOnTrackEnd, SignalPlayStatus,
        SignalSetSongPlayStatusRequest,
    },
};

pub struct PlayerActor {
    tasks: JoinSet<()>,
    player: Arc<Mutex<MmlPlayer>>,
}
impl Actor for PlayerActor {}
impl ActorName for PlayerActor {
    fn get_name() -> &'static str {
        "PlayerActor"
    }
}
impl ListenDartSignal for PlayerActor {}
impl PlayerActor {
    pub fn try_new() -> Result<Self> {
        Ok(Self {
            tasks: JoinSet::new(),
            player: Arc::new(Mutex::new(MmlPlayer::new(MmlPlayerOptions {
                soundfont_path: Vec::new(),
            })?)),
        })
    }

    pub fn spawn() -> Result<Address<Self>> {
        info!("PlayerActor: Spawning actor and listeners");
        let context = Context::new();
        let addr = context.address();
        let mut actor = Self::try_new()?;

        actor.tasks.spawn(Self::listen_without_response::<
            SignalSetSongPlayStatusRequest,
            _,
        >(addr.clone(), "Set Song Playback Status"));

        actor.tasks.spawn(Self::listen_binary::<
            SignalLoadSoundfontRequest,
            LoadSoundfontRequest,
            _,
        >(addr.clone(), "Load Soundfont"));

        info!("PlayerActor: All listeners spawned");
        spawn(context.run(actor));
        Ok(addr)
    }

    pub async fn play(&mut self) -> Result<()> {
        let note_on_callback: Arc<fn(NoteOnCallbackData)> = Arc::new(|data: NoteOnCallbackData| {
            SignalMmlNoteOn {
                track_index: data.track_index as u32,
                char_index: data.char_index as u32,
                char_length: data.char_length as u32,
            }
            .send_signal_to_dart();
        });

        let track_end_callback: Arc<fn(usize)> = Arc::new(|track_index| {
            SignalOnTrackEnd {
                track_index: track_index as u32,
            }
            .send_signal_to_dart();
        });

        let mut player = self.player.lock().await;
        player.play(Some(note_on_callback), Some(track_end_callback))
    }
}

#[async_trait]
impl Handler<Vec<MmlTrack>> for PlayerActor {
    type Result = Result<()>;
    async fn handle(&mut self, tracks: Vec<MmlTrack>, _: &Context<Self>) -> Result<()> {
        let mut player = self.player.lock().await;
        player.parse_mmls(
            tracks
                .into_iter()
                .map(|t| (t.to_mml(), t.instrument))
                .collect(),
        )?;
        Ok(())
    }
}

#[async_trait]
impl Handler<SignalSetSongPlayStatusRequest> for PlayerActor {
    type Result = Result<()>;
    async fn handle(
        &mut self,
        SignalSetSongPlayStatusRequest { status }: SignalSetSongPlayStatusRequest,
        _: &Context<Self>,
    ) -> Result<()> {
        match status {
            SignalPlayStatus::Play => self.play().await?,
            SignalPlayStatus::Pause => {
                let mut player = self.player.lock().await;
                player.pause()?;
            }
            SignalPlayStatus::Stop => {
                let mut player = self.player.lock().await;
                player.stop()?;
            }
        }
        Ok(())
    }
}

pub(super) struct LoadSoundfontRequest(Vec<u8>);
impl From<Vec<u8>> for LoadSoundfontRequest {
    fn from(value: Vec<u8>) -> Self {
        Self(value)
    }
}

#[async_trait]
impl Handler<LoadSoundfontRequest> for PlayerActor {
    type Result = Result<()>;
    async fn handle(
        &mut self,
        LoadSoundfontRequest(bytes): LoadSoundfontRequest,
        _: &Context<Self>,
    ) -> Result<()> {
        let mut player = self.player.lock().await;
        player.load_soundfont_from_bytes(bytes)
    }
}
