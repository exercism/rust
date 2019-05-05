use acronym;

#[test]
fn empty() { assert_eq!("", acronym::abbreviate("")); }

#[test]
#[ignore]
fn basic() {
    assert_eq!("PNG", acronym::abbreviate("Portable Network Graphics"));
}

#[test]
#[ignore]
fn lowercase_words() {
    assert_eq!("ROR", acronym::abbreviate("Ruby on Rails"));
}

#[test]
#[ignore]
fn camelcase() {
    assert_eq!("HTML", acronym::abbreviate("HyperText Markup Language"));
}

#[test]
#[ignore]
fn punctuation() {
    assert_eq!("FIFO", acronym::abbreviate("First In, First Out"));
}

#[test]
#[ignore]
fn all_caps_words() {
    assert_eq!("PHP", acronym::abbreviate("PHP: Hypertext Preprocessor"));
}

#[test]
#[ignore]
fn non_acronym_all_caps_word() {
    assert_eq!(
        "GIMP",
        acronym::abbreviate("GNU Image Manipulation Program")
    )
}

#[test]
#[ignore]
fn hyphenated() {
    assert_eq!(
        "CMOS",
        acronym::abbreviate("Complementary metal-oxide semiconductor")
    )
}
