use revelation_mobile_midi_to_mml::Song;
use tokio::sync::Mutex;

pub struct State {
    song: Option<Song>,
}

pub static STATE: Mutex<State> = Mutex::const_new(State {
    song: None,
});

pub async fn set_temp_song(song: Song) {
    let mut state = STATE.lock().await;
    state.song = Some(song);
}
