use std::fmt::Display;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
#[derive(serde::Deserialize, serde::Serialize)]
pub enum BaseNote {
    A,
    B,
    #[default]
    C,
    D,
    E,
    F,
    G,
}

impl Display for BaseNote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BaseNote::A => "A",
                BaseNote::B => "B",
                BaseNote::C => "C",
                BaseNote::D => "D",
                BaseNote::E => "E",
                BaseNote::F => "F",
                BaseNote::G => "G",
            }
        )
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
#[derive(serde::Deserialize, serde::Serialize)]
pub enum Modifier {
    #[default]
    None,
    Sharp,
    Flat,
}

impl Display for Modifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Modifier::None => "",
                Modifier::Sharp => "#",
                Modifier::Flat => "b",
            }
        )
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Note {
    pub base_note: BaseNote,
    pub modifier: Modifier,
}

impl Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.base_note, self.modifier)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
#[derive(serde::Deserialize, serde::Serialize)]
pub enum ScaleStep {
    #[default]
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
}

impl Display for ScaleStep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.num())
    }
}

impl ScaleStep {
    pub fn num(&self) -> u8 {
        match self {
            ScaleStep::First => 1,
            ScaleStep::Second => 2,
            ScaleStep::Third => 3,
            ScaleStep::Fourth => 4,
            ScaleStep::Fifth => 5,
            ScaleStep::Sixth => 6,
            ScaleStep::Seventh => 7,
        }
    }

    pub fn ord(&self) -> usize {
        match self {
            ScaleStep::First => 0,
            ScaleStep::Second => 1,
            ScaleStep::Third => 2,
            ScaleStep::Fourth => 3,
            ScaleStep::Fifth => 4,
            ScaleStep::Sixth => 5,
            ScaleStep::Seventh => 6,
        }
    }
}

pub const ALL_SCALE_STEPS: [ScaleStep; 7] = [
    ScaleStep::First,
    ScaleStep::Second,
    ScaleStep::Third,
    ScaleStep::Fourth,
    ScaleStep::Fifth,
    ScaleStep::Sixth,
    ScaleStep::Seventh,
];

pub const SCALE_STEPS_WEIGHTS: [usize; 7] = [0, 3, 3, 9, 9, 9, 1];

const A: Note = Note {
    base_note: BaseNote::A,
    modifier: Modifier::None,
};
const B: Note = Note {
    base_note: BaseNote::B,
    modifier: Modifier::None,
};
const C: Note = Note {
    base_note: BaseNote::C,
    modifier: Modifier::None,
};
const D: Note = Note {
    base_note: BaseNote::D,
    modifier: Modifier::None,
};
const E: Note = Note {
    base_note: BaseNote::E,
    modifier: Modifier::None,
};
const F: Note = Note {
    base_note: BaseNote::F,
    modifier: Modifier::None,
};
const G: Note = Note {
    base_note: BaseNote::G,
    modifier: Modifier::None,
};

const A_FLAT: Note = Note {
    base_note: BaseNote::A,
    modifier: Modifier::Flat,
};
const B_FLAT: Note = Note {
    base_note: BaseNote::B,
    modifier: Modifier::Flat,
};
const C_FLAT: Note = Note {
    base_note: BaseNote::C,
    modifier: Modifier::Flat,
};
const D_FLAT: Note = Note {
    base_note: BaseNote::D,
    modifier: Modifier::Flat,
};
const E_FLAT: Note = Note {
    base_note: BaseNote::E,
    modifier: Modifier::Flat,
};
#[allow(dead_code)]
const F_FLAT: Note = Note {
    base_note: BaseNote::F,
    modifier: Modifier::Flat,
};
const G_FLAT: Note = Note {
    base_note: BaseNote::G,
    modifier: Modifier::Flat,
};

const A_SHARP: Note = Note {
    base_note: BaseNote::A,
    modifier: Modifier::Sharp,
};
#[allow(dead_code)]
const B_SHARP: Note = Note {
    base_note: BaseNote::B,
    modifier: Modifier::Sharp,
};
const C_SHARP: Note = Note {
    base_note: BaseNote::C,
    modifier: Modifier::Sharp,
};
const D_SHARP: Note = Note {
    base_note: BaseNote::D,
    modifier: Modifier::Sharp,
};
const E_SHARP: Note = Note {
    base_note: BaseNote::E,
    modifier: Modifier::Sharp,
};
const F_SHARP: Note = Note {
    base_note: BaseNote::F,
    modifier: Modifier::Sharp,
};
const G_SHARP: Note = Note {
    base_note: BaseNote::G,
    modifier: Modifier::Sharp,
};

pub const ALL_NOTES: [Note; 21] = [
    A, B, C, D, E, F, G, A_FLAT, B_FLAT, C_FLAT, D_FLAT, E_FLAT, F_FLAT, G_FLAT, A_SHARP, B_SHARP,
    C_SHARP, D_SHARP, E_SHARP, F_SHARP, G_SHARP,
];

pub type Scale = [Note; 7];

const C_SCALE: Scale = [C, D, E, F, G, A, B];
const G_SCALE: Scale = [G, A, B, C, D, E, F_SHARP];
const D_SCALE: Scale = [D, E, F_SHARP, G, A, B, C_SHARP];
const A_SCALE: Scale = [A, B, C_SHARP, D, E, F_SHARP, G_SHARP];
const E_SCALE: Scale = [E, F_SHARP, G_SHARP, A, B, C_SHARP, D_SHARP];
const B_SCALE: Scale = [B, C_SHARP, D_SHARP, E, F_SHARP, G_SHARP, A_SHARP];
const F_SHARP_SCALE: Scale = [F_SHARP, G_SHARP, A_SHARP, B, C_SHARP, D_SHARP, E_SHARP];
const F_SCALE: Scale = [F, G, A, B_FLAT, C, D, E];
const B_FLAT_SCALE: Scale = [B_FLAT, C, D, E_FLAT, F, G, A];
const E_FLAT_SCALE: Scale = [E_FLAT, F, G, A_FLAT, B_FLAT, C, D];
const A_FLAT_SCALE: Scale = [A_FLAT, B_FLAT, C, D_FLAT, E_FLAT, F, G];
const D_FLAT_SCALE: Scale = [D_FLAT, E_FLAT, F, G_FLAT, A_FLAT, B_FLAT, C];
const G_FLAT_SCALE: Scale = [G_FLAT, A_FLAT, B_FLAT, C_FLAT, D_FLAT, E_FLAT, F];

pub const ALL_SCALES: [Scale; 13] = [
    C_SCALE,
    G_SCALE,
    D_SCALE,
    A_SCALE,
    E_SCALE,
    B_SCALE,
    F_SHARP_SCALE,
    F_SCALE,
    B_FLAT_SCALE,
    E_FLAT_SCALE,
    A_FLAT_SCALE,
    D_FLAT_SCALE,
    G_FLAT_SCALE,
];
pub const ALL_SCALES_WEIGHTED: [usize; 13] = [
    9, // C
    8, // G
    7, // D
    6, // A
    5, // E
    4, // B
    3, // Fb
    8, // F
    7, // Bb
    6, // Eb
    5, // Ab
    4, // Db
    3, // Gb
];
