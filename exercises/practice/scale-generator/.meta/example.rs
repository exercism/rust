#[derive(Debug)]
pub struct Error;

pub struct Scale(Vec<String>);

const NUM_TONICS: usize = 12;

const SCALE_WITH_SHARPS: [&str; NUM_TONICS] = [
    "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#",
];

const SCALE_WITH_FLATS: [&str; NUM_TONICS] = [
    "A", "Bb", "B", "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab",
];

const USING_FLATS: [&str; 12] = [
    "F", "Bb", "Eb", "Ab", "Db", "Gb", "d", "g", "c", "f", "bb", "eb",
];

fn get_uppercase_tonic(tonic: &str) -> String {
    let mut iter = tonic.chars();
    let mut s = String::new();

    let first = iter.next().unwrap().to_uppercase().next().unwrap();

    s.push(first);
    s.extend(iter);
    s
}

impl Scale {
    pub fn new(tonic: &str, intervals: &str) -> Result<Scale, Error> {
        let scale = if USING_FLATS.contains(&tonic) {
            &SCALE_WITH_FLATS
        } else {
            &SCALE_WITH_SHARPS
        };

        let tonic = get_uppercase_tonic(tonic);

        let mut tonics_iter = scale
            .iter()
            .cycle()
            .skip_while(|&&t| t != tonic)
            .map(|&t| t.to_owned());

        let mut v = vec![tonics_iter.next().unwrap()];

        for interv in intervals.bytes() {
            match interv {
                b'm' => v.push(tonics_iter.next().unwrap()),
                b'M' => v.push(tonics_iter.nth(1).unwrap()),
                b'A' => v.push(tonics_iter.nth(2).unwrap()),
                _ => panic!("unknown interval"),
            }
        }

        Ok(Scale(v))
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        Self::new(tonic, "mmmmmmmmmmmm")
    }

    pub fn enumerate(&self) -> Vec<String> {
        self.0.clone()
    }
}
