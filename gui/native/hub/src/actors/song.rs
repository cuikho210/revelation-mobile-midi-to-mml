use anyhow::{Context as AnyhowContext, Result};
use async_trait::async_trait;
use messages::{
    actor::Actor,
    prelude::{Address, Context, Handler},
};
use midi_to_mml::{MmlSong, MmlSongOptions};
use tokio::{spawn, task::JoinSet};
use tracing::info;

use crate::{
    actors::{
        common::{ActorName, ListenDartSignal},
        player::PlayerActor,
    },
    signal_converter,
    signals::{
        error::ErrorFrom,
        song::{
            SignalApplyKeymap, SignalEqualizeTracksRequest, SignalLoadSongFromPathRequest,
            SignalLoadSongFromPathResponse, SignalMergeTracksRequest, SignalMmlSongStatus,
            SignalRenameTrackRequest, SignalSplitTrackRequest, SignalUpdateMmlSongOptionsRequest,
            SignalUpdateMmlTracks,
        },
    },
};

pub struct SongActor {
    tasks: JoinSet<()>,
    song: Option<MmlSong>,
    player: Address<PlayerActor>,
}
impl Actor for SongActor {}
impl ActorName for SongActor {
    fn get_name() -> &'static str {
        "SongActor"
    }
}
impl ListenDartSignal for SongActor {}
impl SongActor {
    pub fn new(player: Address<PlayerActor>) -> Self {
        Self {
            tasks: JoinSet::new(),
            song: None,
            player,
        }
    }

    pub fn spawn(player: Address<PlayerActor>) -> Address<Self> {
        info!("SongActor: Spawning actor and listeners");
        let context = Context::new();
        let addr = context.address();
        let mut actor = Self::new(player);

        actor.tasks.spawn(Self::listen_with_response::<
            SignalLoadSongFromPathRequest,
            _,
            _,
        >(
            addr.clone(), "Load Song", Some(ErrorFrom::LoadSong)
        ));
        actor.tasks.spawn(Self::listen_with_response::<
            SignalUpdateMmlSongOptionsRequest,
            _,
            _,
        >(
            addr.clone(),
            "Update Song Options",
            Some(ErrorFrom::LoadSong),
        ));
        actor
            .tasks
            .spawn(Self::listen_with_response::<SignalSplitTrackRequest, _, _>(
                addr.clone(),
                "Split Track",
                Some(ErrorFrom::SplitTrack),
            ));
        actor.tasks.spawn(
            Self::listen_with_response::<SignalMergeTracksRequest, _, _>(
                addr.clone(),
                "Merge Tracks",
                Some(ErrorFrom::MergeTracks),
            ),
        );
        actor.tasks.spawn(Self::listen_with_response::<
            SignalEqualizeTracksRequest,
            _,
            _,
        >(
            addr.clone(),
            "Equalize Tracks",
            Some(ErrorFrom::EqualizeTracks),
        ));
        actor.tasks.spawn(
            Self::listen_with_response::<SignalRenameTrackRequest, _, _>(
                addr.clone(),
                "Rename Track",
                Some(ErrorFrom::RenameTrack),
            ),
        );
        actor
            .tasks
            .spawn(Self::listen_with_response::<SignalApplyKeymap, _, _>(
                addr.clone(),
                "Apply Keymap",
                Some(ErrorFrom::ApplyKeymap),
            ));

        spawn(context.run(actor));
        addr
    }
}

#[async_trait]
impl Handler<SignalLoadSongFromPathRequest> for SongActor {
    type Result = Result<SignalLoadSongFromPathResponse>;
    async fn handle(
        &mut self,
        request: SignalLoadSongFromPathRequest,
        _: &Context<Self>,
    ) -> Result<SignalLoadSongFromPathResponse> {
        let song = MmlSong::from_path(request.path, MmlSongOptions::default())?;
        let song_options = signal_converter::mml_song_options_to_signal(&song.options);
        let tracks = signal_converter::mml_song_tracks_to_signal(&song.tracks);
        self.player.send(song.tracks.clone()).await??;
        self.song = Some(song);
        Ok(SignalLoadSongFromPathResponse {
            song_status: Some(SignalMmlSongStatus {
                tracks,
                song_options,
            }),
        })
    }
}

#[async_trait]
impl Handler<SignalUpdateMmlSongOptionsRequest> for SongActor {
    type Result = Result<SignalUpdateMmlTracks>;
    async fn handle(
        &mut self,
        request: SignalUpdateMmlSongOptionsRequest,
        _: &Context<Self>,
    ) -> Result<SignalUpdateMmlTracks> {
        let song = self.song.as_mut().context("Song is None")?;
        song.set_song_options(signal_converter::signal_to_mml_song_options(
            &request.song_options,
        ))?;
        self.player.send(song.tracks.clone()).await??;
        let tracks = signal_converter::mml_song_tracks_to_signal(&song.tracks);
        Ok(SignalUpdateMmlTracks { tracks })
    }
}

#[async_trait]
impl Handler<SignalSplitTrackRequest> for SongActor {
    type Result = Result<SignalUpdateMmlTracks>;
    async fn handle(
        &mut self,
        request: SignalSplitTrackRequest,
        _: &Context<Self>,
    ) -> Result<SignalUpdateMmlTracks> {
        let song = self.song.as_mut().context("Song is None")?;
        song.split_track(request.index as usize)?;
        self.player.send(song.tracks.clone()).await??;
        let tracks = signal_converter::mml_song_tracks_to_signal(&song.tracks);
        Ok(SignalUpdateMmlTracks { tracks })
    }
}

#[async_trait]
impl Handler<SignalMergeTracksRequest> for SongActor {
    type Result = Result<SignalUpdateMmlTracks>;
    async fn handle(
        &mut self,
        SignalMergeTracksRequest { index_a, index_b }: SignalMergeTracksRequest,
        _: &Context<Self>,
    ) -> Result<SignalUpdateMmlTracks> {
        let song = self.song.as_mut().context("Song is None")?;
        song.merge_tracks(index_a as usize, index_b as usize)?;
        self.player.send(song.tracks.clone()).await??;
        let tracks = signal_converter::mml_song_tracks_to_signal(&song.tracks);
        Ok(SignalUpdateMmlTracks { tracks })
    }
}

#[async_trait]
impl Handler<SignalEqualizeTracksRequest> for SongActor {
    type Result = Result<SignalUpdateMmlTracks>;
    async fn handle(
        &mut self,
        SignalEqualizeTracksRequest { index_a, index_b }: SignalEqualizeTracksRequest,
        _: &Context<Self>,
    ) -> Result<SignalUpdateMmlTracks> {
        let song = self.song.as_mut().context("Song is None")?;
        song.equalize_tracks(index_a as usize, index_b as usize)?;
        self.player.send(song.tracks.clone()).await??;
        let tracks = signal_converter::mml_song_tracks_to_signal(&song.tracks);
        Ok(SignalUpdateMmlTracks { tracks })
    }
}

#[async_trait]
impl Handler<SignalRenameTrackRequest> for SongActor {
    type Result = Result<SignalUpdateMmlTracks>;
    async fn handle(
        &mut self,
        SignalRenameTrackRequest { index, name }: SignalRenameTrackRequest,
        _: &Context<Self>,
    ) -> Result<SignalUpdateMmlTracks> {
        let song = self.song.as_mut().context("Song is None")?;
        let track = song
            .tracks
            .get_mut(index as usize)
            .with_context(|| format!("Track at {index} is None"))?;
        track.name = name;
        let tracks = signal_converter::mml_song_tracks_to_signal(&song.tracks);
        Ok(SignalUpdateMmlTracks { tracks })
    }
}

#[async_trait]
impl Handler<SignalApplyKeymap> for SongActor {
    type Result = Result<SignalUpdateMmlTracks>;
    async fn handle(
        &mut self,
        SignalApplyKeymap {
            keymap,
            track_index,
        }: SignalApplyKeymap,
        _: &Context<Self>,
    ) -> Result<SignalUpdateMmlTracks> {
        let song = self.song.as_mut().context("Song is None")?;
        song.apply_keymap(track_index as usize, &keymap);
        let tracks = signal_converter::mml_song_tracks_to_signal(&song.tracks);
        Ok(SignalUpdateMmlTracks { tracks })
    }
}
