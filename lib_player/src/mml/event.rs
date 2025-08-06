use crate::NoteEvent;

pub enum MmlEvent {
    SetTempo(usize),
    SetVelocity(u8),
    SetOctave(u8),
    IncreOctave,
    DecreOctave,
    ConnectChord,
    SetNote(NoteEvent),
    Empty,
}
