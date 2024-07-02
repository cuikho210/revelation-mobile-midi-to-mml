use crate::note_event::NoteEvent;

pub enum MmlEvent {
    SetTempo(u16),
    SetVelocity(u8),
    SetOctave(u8),
    IncreOctave,
    DecreOctave,
    ConnectChord,
    SetNote(NoteEvent),
}
