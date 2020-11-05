use rstest::rstest;
use rstest_reuse::{self, *};
#[cfg(feature = "io")]
use std::io::{Read, Write};
use xorcism::Xorcism;

#[test]
fn identity() {
    let mut xs = Xorcism::new(&[0]);
    let data = "This is super-secret, cutting edge encryption, folks.";

    assert_eq!(
        xs.munge(data.as_bytes()).collect::<Vec<_>>(),
        data.as_bytes()
    );
}

#[template]
#[rstest(
    key,
    input,
    case::key_shorter_than_data("abcde", "123455"),
    case::key_len_equal_to_data(
        "The quick brown fox jumped over the lazy dog.",
        "Wait, oops, this is not the pangram exercise!",
    ),
    case::key_longer_than_data(
        "A properly cryptographically random key longer than the data can actually be fairly secure.",
        "Text is not cryptographically random.",
    ),
    case::shakespearean(
        "Forsooth, let us never break our trust!",
        "The sacred brothership in which we share shall never from our hearts be lost.",
    ),
    case::comics(
        "Who knows what evil lurks in the hearts of men?",
        "Spiderman! It's spiderman! Not a bird, or a plane, or a fireman! Just spiderman!",
    ),
    case::mad_science(
        "TRANSMUTATION_NOTES_1",
        "If wishes were horses, beggars would ride.",
    ),
    case::metaphor(
        "Contextualism",
        "The globe is text, its people prose; all the world's a page."
    ),
    case::emoji("🔑🗝️… 🎹?", "⌨️! 🔒+💻+🧠=🔓")
)]
// note the emoji case above. Most exercism tests don't test emoji, because we like to use strings
// as input, but in this case, they're fine: they're arbitrary binary data that _just happen_ to
// represent emoji when construed as a string, in this case.
fn ref_str(key: &str, input: &str) {}

mod str_str {
    use super::*;

    #[apply(ref_str)]
    fn munge_in_place(key: &str, input: &str) {
        let mut input = input.as_bytes().to_vec();
        let original = input.clone();

        // in-place munging is stateful on Xorcism, so clone it
        // to ensure the keys positions stay synchronized
        let mut xorcism1 = Xorcism::new(key);
        let mut xorcism2 = xorcism1.clone();

        xorcism1.munge_in_place(&mut input);
        assert_eq!(input.len(), original.len());
        assert_ne!(input, original);
        xorcism2.munge_in_place(&mut input);
        assert_eq!(input, original);
    }

    #[apply(ref_str)]
    fn munges(key: &str, input: &str) {
        let mut xorcism = Xorcism::new(key);
        let result: Vec<u8> = xorcism.munge(input.as_bytes()).collect();
        assert_eq!(input.len(), result.len());
        assert_ne!(input.as_bytes(), result);
    }

    #[apply(ref_str)]
    fn round_trip(key: &str, input: &str) {
        let mut xorcism1 = Xorcism::new(key);
        let mut xorcism2 = xorcism1.clone();
        let munge_iter = xorcism1.munge(input.as_bytes());
        let result: Vec<u8> = xorcism2.munge(munge_iter).collect();
        assert_eq!(input.as_bytes(), result);
    }
}

#[test]
#[cfg(feature = "io")]
fn writer_roundtrip() {
    let data = "Spiderman! It's spiderman! Not a bird, or a plane, or a fireman! Just spiderman!";
    let mut writer_dest = Vec::new();
    let xs1 = Xorcism::new("Who knows what evil lurks in the hearts of men?");
    let xs2 = xs1.clone();
    {
        let mut writer = xs1.writer(xs2.writer(&mut writer_dest));
        assert!(writer.write_all(data.as_bytes()).is_ok());
    }

    assert_eq!(writer_dest, data.as_bytes());
}

#[test]
#[cfg(feature = "io")]
fn writer_munges() {
    let data = "If wishes were horses, beggars would ride.";
    let mut writer_dest = Vec::new();
    {
        let mut writer = Xorcism::new("TRANSMUTATION_NOTES_1").writer(&mut writer_dest);
        assert!(writer.write_all(data.as_bytes()).is_ok());
    }

    assert_eq!(writer_dest.len(), data.len());
    assert_ne!(writer_dest, data.as_bytes());
}

#[test]
#[cfg(feature = "io")]
fn reader_munges() {
    let data = "The globe is text, its people prose; all the world's a page.";
    let mut reader = Xorcism::new("But who owns the book?").reader(data.as_bytes());
    let mut buf = Vec::with_capacity(data.len());
    let bytes_read = reader.read_to_end(&mut buf).unwrap();
    assert_eq!(bytes_read, data.len());
    assert_ne!(buf, data.as_bytes());
}

#[test]
#[cfg(feature = "io")]
fn reader_roundtrip() {
    let data = "Mary Poppins was a kind witch. She cared for the children.";
    let key = "supercalifragilisticexpialidocious.";
    let xs = Xorcism::new(key);
    let mut reader = xs.clone().reader(xs.reader(data.as_bytes()));
    let mut buf = Vec::with_capacity(data.len());
    let bytes_read = reader.read_to_end(&mut buf).unwrap();
    assert_eq!(bytes_read, data.len());
    assert_eq!(buf, data.as_bytes());
}
