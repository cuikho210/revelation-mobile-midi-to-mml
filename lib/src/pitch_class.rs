use std::fmt::Display;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PitchClass {
    C,
    Db,
    D,
    Eb,
    E,
    F,
    Gb,
    G,
    Ab,
    A,
    Bb,
    B,
    Rest,
}

impl Display for PitchClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_str = match self {
            Self::C => "c",
            Self::Db => "c+",
            Self::D => "d",
            Self::Eb => "d+",
            Self::E => "e",
            Self::F => "f",
            Self::Gb => "f+",
            Self::G => "g",
            Self::Ab => "g+",
            Self::A => "a",
            Self::Bb => "a+",
            Self::B => "b",
            Self::Rest => "r",
        };

        write!(f, "{}", display_str)
    }
}
