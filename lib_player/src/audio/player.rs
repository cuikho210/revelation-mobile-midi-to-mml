use crate::{NoteEvent, NoteOnCallbackData, Parser, PlaybackStatus, SynthOutputConnection, utils};
use anyhow::{Context, Result};
use midi_to_mml::Instrument;
use std::{
    sync::{Arc, RwLock},
    thread::sleep,
    time::{Duration, Instant},
};

pub struct TrackPlayer {
    pub index: usize,
    pub parser: Parser,
    pub playback_status: Arc<RwLock<PlaybackStatus>>,
    pub instrument: Instrument,
    pub connection: SynthOutputConnection,

    note_before: Option<NoteEvent>,
    current_chord: Vec<NoteEvent>,
    absolute_duration: isize,
    current_note_index: usize,
    note_on_callback: Option<Arc<fn(NoteOnCallbackData)>>,
    track_end_callback: Option<Arc<fn(usize)>>,
}
impl TrackPlayer {
    pub fn from_parser(
        index: usize,
        parser: Parser,
        playback_status: Arc<RwLock<PlaybackStatus>>,
        instrument: Instrument,
        mut connection: SynthOutputConnection,
    ) -> Result<Self> {
        connection.program_change(instrument.midi_channel, instrument.instrument_id)?;

        Ok(Self {
            index,
            parser,
            playback_status,
            instrument,
            connection,
            note_before: None,
            current_chord: Vec::new(),
            absolute_duration: 0,
            current_note_index: 0,
            note_on_callback: None,
            track_end_callback: None,
        })
    }

    pub fn play(
        &mut self,
        time_start: Instant,
        note_on_callback: Option<Arc<fn(NoteOnCallbackData)>>,
        track_end_callback: Option<Arc<fn(usize)>>,
    ) -> Result<()> {
        self.note_on_callback = note_on_callback;
        self.track_end_callback = track_end_callback;
        self.play_notes_linear(time_start)
    }

    fn reset_state(&mut self) {
        self.current_note_index = 0;
        self.absolute_duration = 0;
        self.note_before = None;
        self.current_chord = Vec::new();
    }

    fn stop_all_notes(&mut self) -> Result<()> {
        self.connection.all_notes_off(self.instrument.midi_channel)
    }

    fn handle_playback_status(&mut self) -> Result<PlaybackStatus> {
        let playback_status = self.playback_status.clone();

        if let Ok(guard) = playback_status.read()
            && *guard != PlaybackStatus::PLAY
        {
            if *guard == PlaybackStatus::PAUSE {
                return Ok(PlaybackStatus::PAUSE);
            } else {
                self.reset_state();
                self.stop_all_notes()?;

                return Ok(PlaybackStatus::STOP);
            }
        }

        Ok(PlaybackStatus::PLAY)
    }

    fn handle_note_blocking(&mut self, duration: Duration) -> Result<()> {
        let time_start = Instant::now();
        let time_loop = Duration::from_millis(32);

        loop {
            let elapsed = time_start.elapsed();
            if elapsed >= duration {
                break;
            }

            let remaining = duration - elapsed;
            let sleep_duration = if remaining > time_loop {
                time_loop
            } else {
                remaining
            };

            if self.handle_playback_status()? != PlaybackStatus::PLAY {
                break;
            }

            sleep(sleep_duration);
        }

        Ok(())
    }

    fn play_notes_linear(&mut self, time_start: Instant) -> Result<()> {
        let connection = self.connection.clone();

        for index in self.current_note_index..self.parser.notes.len() {
            self.current_note_index = index;

            if self.handle_playback_status()? != PlaybackStatus::PLAY {
                return Ok(());
            }

            let note = self
                .parser
                .notes
                .get(index)
                .context("Failed to get note")?
                .to_owned();

            if note.is_connected_to_prev_note {
                if let Some(before_note) = self.note_before.as_ref()
                    && self.current_chord.is_empty()
                {
                    self.current_chord.push(before_note.to_owned());
                }

                self.current_chord.push(note.to_owned());
                continue;
            }

            let correct_duration = time_start.elapsed().as_millis() as isize;
            let duration_diff = correct_duration - self.absolute_duration;

            if !self.current_chord.is_empty() {
                let chord_duration = utils::get_longest_note_duration(&self.current_chord);
                let duration = chord_duration - duration_diff;

                if duration <= 0 {
                    println!(
                        "[track_player.play_notes_linear] skip by duration is {} ms",
                        duration
                    );
                    continue;
                }

                let duration = Duration::from_millis(duration as u64);

                send_note_on_event_from_chord(
                    &self.note_on_callback,
                    &self.current_chord,
                    self.index,
                )?;

                let sleep_duration = utils::play_chord(
                    connection.to_owned(),
                    &self.current_chord,
                    self.instrument.midi_channel,
                    Some(duration),
                )?;

                self.handle_note_blocking(sleep_duration)?;

                utils::stop_chord(
                    connection.to_owned(),
                    &self.current_chord,
                    self.instrument.midi_channel,
                )?;

                self.absolute_duration += chord_duration;
                self.current_chord.clear();
                self.note_before = Some(note.to_owned());

                continue;
            }

            if let Some(before_note) = self.note_before.as_ref() {
                let note_duration = before_note.duration_in_ms as isize;
                let duration = note_duration - duration_diff;

                if duration <= 0 {
                    println!(
                        "[track_player.play_notes_linear] skip by duration is {} ms",
                        duration
                    );
                    continue;
                }

                let duration = Duration::from_millis(duration as u64);

                send_note_on_event_from_note(&self.note_on_callback, before_note, self.index);

                let sleep_duration = utils::play_note(
                    connection.to_owned(),
                    before_note,
                    self.instrument.midi_channel,
                    Some(duration),
                )?;

                let before_note = before_note.to_owned();
                self.handle_note_blocking(sleep_duration)?;

                utils::stop_note(
                    connection.to_owned(),
                    &before_note,
                    self.instrument.midi_channel,
                )?;

                self.absolute_duration += note_duration;
            }

            self.note_before = Some(note.to_owned());
            continue;
        }

        if !self.current_chord.is_empty() {
            send_note_on_event_from_chord(&self.note_on_callback, &self.current_chord, self.index)?;

            let sleep_duration = utils::play_chord(
                connection.to_owned(),
                &self.current_chord,
                self.instrument.midi_channel,
                None,
            )?;

            self.handle_note_blocking(sleep_duration)?;

            utils::stop_chord(
                connection.to_owned(),
                &self.current_chord,
                self.instrument.midi_channel,
            )?;
        }

        if let Some(before_note) = self.note_before.as_ref() {
            send_note_on_event_from_note(&self.note_on_callback, before_note, self.index);

            let sleep_duration = utils::play_note(
                connection.to_owned(),
                before_note,
                self.instrument.midi_channel,
                None,
            )?;

            let before_note = before_note.to_owned();
            self.handle_note_blocking(sleep_duration)?;

            utils::stop_note(
                connection.to_owned(),
                &before_note,
                self.instrument.midi_channel,
            )?;
        }

        self.reset_state();

        if let Some(callback) = self.track_end_callback.as_ref() {
            callback(self.index);
        }

        Ok(())
    }
}

fn send_note_on_event_from_note(
    note_on_tx: &Option<Arc<fn(NoteOnCallbackData)>>,
    note: &NoteEvent,
    track_index: usize,
) {
    if let Some(callback) = note_on_tx {
        callback(NoteOnCallbackData {
            track_index,
            char_index: note.char_index,
            char_length: note.char_length,
        });
    }
}

fn send_note_on_event_from_chord(
    note_on_callback: &Option<Arc<fn(NoteOnCallbackData)>>,
    chord: &[NoteEvent],
    track_index: usize,
) -> Result<()> {
    if let Some(callback) = note_on_callback {
        let first_note = chord.first().context("Failed to get first note of chord")?;
        let last_note = chord.last().context("Failed to get last note of chord")?;

        let end_at = last_note.char_index + last_note.char_length;
        let char_index = first_note.char_index;
        let char_length = end_at - char_index;

        callback(NoteOnCallbackData {
            track_index,
            char_index,
            char_length,
        });
    }

    Ok(())
}
