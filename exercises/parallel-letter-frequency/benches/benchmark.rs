#![feature(test)]
extern crate parallel_letter_frequency;
extern crate test;

use std::collections::HashMap;
use test::Bencher;

#[bench]
fn bench_tiny_parallel(b: &mut Bencher) {
    let tiny = &["a"];
    b.iter(|| parallel_letter_frequency::frequency(tiny, 3));
}

#[bench]
fn bench_tiny_sequential(b: &mut Bencher) {
    let tiny = &["a"];
    b.iter(|| frequency(tiny));
}

#[bench]
fn bench_small_parallel(b: &mut Bencher) {
    let texts = all_texts(1);
    b.iter(|| parallel_letter_frequency::frequency(&texts, 3));
}

#[bench]
fn bench_small_sequential(b: &mut Bencher) {
    let texts = all_texts(1);
    b.iter(|| frequency(&texts));
}

#[bench]
fn bench_large_parallel(b: &mut Bencher) {
    let texts = all_texts(30);
    b.iter(|| parallel_letter_frequency::frequency(&texts, 3));
}

#[bench]
fn bench_large_sequential(b: &mut Bencher) {
    let texts = all_texts(30);
    b.iter(|| frequency(&texts));
}

/// Simple sequential char frequency. Can it be beat?
pub fn frequency(texts: &[&str]) -> HashMap<char, usize> {
    let mut map = HashMap::new();

    for line in texts {
        for chr in line.chars().filter(|c| c.is_alphabetic()) {
            if let Some(c) = chr.to_lowercase().next() {
                (*map.entry(c).or_insert(0)) += 1;
            }
        }
    }

    map
}

fn all_texts(repeat: usize) -> Vec<&'static str> {
    [ODE_AN_DIE_FREUDE, WILHELMUS, STAR_SPANGLED_BANNER]
        .iter()
        .cycle()
        .take(3 * repeat)
        .flat_map(|anthem| anthem.iter().cloned())
        .collect()
}

// Poem by Friedrich Schiller. The corresponding music is the European Anthem.
pub const ODE_AN_DIE_FREUDE: [&str; 8] = [
    "Freude schöner Götterfunken",
    "Tochter aus Elysium,",
    "Wir betreten feuertrunken,",
    "Himmlische, dein Heiligtum!",
    "Deine Zauber binden wieder",
    "Was die Mode streng geteilt;",
    "Alle Menschen werden Brüder,",
    "Wo dein sanfter Flügel weilt.",
];

// Dutch national anthem
pub const WILHELMUS: [&str; 8] = [
    "Wilhelmus van Nassouwe",
    "ben ik, van Duitsen bloed,",
    "den vaderland getrouwe",
    "blijf ik tot in den dood.",
    "Een Prinse van Oranje",
    "ben ik, vrij, onverveerd,",
    "den Koning van Hispanje",
    "heb ik altijd geëerd.",
];

// American national anthem
pub const STAR_SPANGLED_BANNER: [&str; 8] = [
    "O say can you see by the dawn's early light,",
    "What so proudly we hailed at the twilight's last gleaming,",
    "Whose broad stripes and bright stars through the perilous fight,",
    "O'er the ramparts we watched, were so gallantly streaming?",
    "And the rockets' red glare, the bombs bursting in air,",
    "Gave proof through the night that our flag was still there;",
    "O say does that star-spangled banner yet wave,",
    "O'er the land of the free and the home of the brave?",
];
