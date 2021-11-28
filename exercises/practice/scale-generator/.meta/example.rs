#[macro_use]
extern crate enum_primitive_derive;
extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate itertools;
extern crate num_traits;

pub use self::interval::{Interval, Intervals};
use self::note::Accidental;
pub use self::note::Note;
use failure::Error;
use std::str::FromStr;

pub mod interval {
    use itertools::Itertools;
    use std::fmt;
    use std::ops::Deref;
    use std::str::FromStr;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Fail)]
    pub enum ParseErr {
        #[fail(display = "invalid interval")]
        InvalidInterval,
        #[fail(display = "wrong number of semitones")]
        WrongNumberOfSemitones,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Primitive)]
    pub enum Interval {
        HalfStep = 1,
        WholeStep = 2,
        AugmentedFirst = 3,
    }

    impl fmt::Display for Interval {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            use self::Interval::*;
            write!(
                f,
                "{}",
                match self {
                    HalfStep => "m",
                    WholeStep => "M",
                    AugmentedFirst => "A",
                }
            )
        }
    }

    impl FromStr for Interval {
        type Err = ParseErr;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            use self::Interval::*;
            match s {
                "m" => Ok(HalfStep),
                "M" => Ok(WholeStep),
                "A" => Ok(AugmentedFirst),
                _ => Err(ParseErr::InvalidInterval),
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Intervals(Vec<Interval>);

    impl fmt::Display for Intervals {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0.iter().join(""))
        }
    }

    impl FromStr for Intervals {
        type Err = ParseErr;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut semitones = Vec::with_capacity(s.len());

            for (i, c) in s.char_indices() {
                semitones.push(Interval::from_str(&s[i..i + c.len_utf8()])?);
            }

            if semitones.iter().take(12).map(|&i| i as u8).sum::<u8>() == 12 {
                Ok(Intervals(semitones))
            } else {
                Err(ParseErr::WrongNumberOfSemitones)
            }
        }
    }

    impl Deref for Intervals {
        type Target = Vec<Interval>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_parse_chromatic() {
            assert!("mmmmmmmmmmmm".parse::<Intervals>().is_ok());
        }

        #[test]
        fn test_parse_major() {
            assert!("MMmMMMm".parse::<Intervals>().is_ok());
        }

        #[test]
        fn test_parse_minor() {
            assert!("MmMMmMM".parse::<Intervals>().is_ok());
        }
    }
}

pub mod note {
    use crate::Interval;
    use num_traits::{FromPrimitive, ToPrimitive};
    use std::fmt;
    use std::ops::AddAssign;
    use std::str::FromStr;

    pub const SEMITONES: i8 = 12;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Primitive)]
    pub enum Semitone {
        A = 0,
        ASharp = 1,
        B = 2,
        C = 3,
        CSharp = 4,
        D = 5,
        DSharp = 6,
        E = 7,
        F = 8,
        FSharp = 9,
        G = 10,
        GSharp = 11,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Primitive)]
    pub enum Root {
        A = 0,
        B = 2,
        C = 3,
        D = 5,
        E = 7,
        F = 8,
        G = 10,
    }

    impl fmt::Display for Root {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Accidental {
        Sharp,
        Flat,
    }

    impl Accidental {
        fn to_i8(&self) -> i8 {
            match *self {
                Accidental::Sharp => 1,
                Accidental::Flat => -1,
            }
        }

        pub fn from_tonic(tonic: &str) -> Accidental {
            match tonic {
                "C" | "a" | "G" | "D" | "A" | "E" | "B" | "F#" | "e" | "b" | "f#" | "c#" | "g#"
                | "d#" => Accidental::Sharp,
                _ => Accidental::Flat,
            }
        }
    }

    impl fmt::Display for Accidental {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "{}",
                match &self {
                    Accidental::Sharp => '#',
                    Accidental::Flat => 'b',
                }
            )
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Note {
        tonic: Root,
        accidental: Option<Accidental>,
    }

    impl Note {
        pub fn canonicalize(&self, lean: Accidental) -> Note {
            let mut n: Note = Semitone::from(*self).into();
            if let Some(accidental) = n.accidental {
                if accidental != lean && lean == Accidental::Flat {
                    n += Interval::HalfStep;
                    n.accidental = Some(Accidental::Flat);
                }
            }
            n
        }
    }

    impl fmt::Display for Note {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "{}{}",
                self.tonic,
                self.accidental.map_or(String::new(), |a| a.to_string()),
            )
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Fail)]
    pub enum ParseErr {
        #[fail(display = "invalid length")]
        InvalidLength,
        #[fail(display = "invalid tonic")]
        InvalidTonic,
        #[fail(display = "invalid accidental")]
        InvalidAccidental,
    }

    impl FromStr for Note {
        type Err = ParseErr;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let lc = s.to_lowercase();
            let mut iter = lc.chars();

            let mut note = match iter.next() {
                Some(c) if ('a'..='g').contains(&c) => Note {
                    tonic: match c {
                        'a' => Root::A,
                        'b' => Root::B,
                        'c' => Root::C,
                        'd' => Root::D,
                        'e' => Root::E,
                        'f' => Root::F,
                        'g' => Root::G,
                        _ => return Err(ParseErr::InvalidTonic),
                    },
                    accidental: None,
                },
                Some(_) => return Err(ParseErr::InvalidTonic),
                None => return Err(ParseErr::InvalidLength),
            };

            match iter.next() {
                Some('b') => note.accidental = Some(Accidental::Flat),
                Some('#') => note.accidental = Some(Accidental::Sharp),
                Some(_) => return Err(ParseErr::InvalidAccidental),
                None => {}
            }

            if iter.next().is_some() {
                return Err(ParseErr::InvalidLength);
            }

            Ok(note)
        }
    }

    impl From<Semitone> for Note {
        fn from(s: Semitone) -> Self {
            Note {
                tonic: match s {
                    Semitone::A | Semitone::ASharp => Root::A,
                    Semitone::B => Root::B,
                    Semitone::C | Semitone::CSharp => Root::C,
                    Semitone::D | Semitone::DSharp => Root::D,
                    Semitone::E => Root::E,
                    Semitone::F | Semitone::FSharp => Root::F,
                    Semitone::G | Semitone::GSharp => Root::G,
                },
                accidental: match s {
                    Semitone::ASharp
                    | Semitone::CSharp
                    | Semitone::DSharp
                    | Semitone::FSharp
                    | Semitone::GSharp => Some(Accidental::Sharp),
                    _ => None,
                },
            }
        }
    }

    impl From<Note> for Semitone {
        fn from(n: Note) -> Self {
            Semitone::from_i8(
                (SEMITONES + n.tonic.to_i8().unwrap() + n.accidental.map_or(0, |a| a.to_i8()))
                    % SEMITONES,
            )
            .expect("must have valid semitone")
        }
    }

    impl AddAssign<Interval> for Note {
        fn add_assign(&mut self, rhs: Interval) {
            *self = Semitone::from_i8(
                (SEMITONES + Semitone::from(*self).to_i8().unwrap() + rhs.to_i8().unwrap())
                    % SEMITONES,
            )
            .unwrap()
            .into();
        }
    }
}

#[derive(Debug)]
pub struct Scale {
    tonic: Note,
    lean: Accidental,
    intervals: Intervals,
}

impl Scale {
    pub fn new(tonic: &str, intervals: &str) -> Result<Scale, Error> {
        Ok(Scale {
            tonic: Note::from_str(tonic)?,
            lean: Accidental::from_tonic(tonic),
            intervals: Intervals::from_str(intervals)?,
        })
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        Scale::new(tonic, "mmmmmmmmmmmm")
    }

    pub fn enumerate(&self) -> Vec<String> {
        let mut out = Vec::with_capacity(self.intervals.len());

        let mut note = self.tonic;
        out.push(note.canonicalize(self.lean).to_string());
        for &interval in self.intervals.iter() {
            note += interval;
            out.push(note.canonicalize(self.lean).to_string());
        }

        out
    }
}
