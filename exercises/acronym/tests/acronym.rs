use acronym;

#[test]
fn empty() {
    assert_eq!(acronym::abbreviate(""), "");
}

#[test]
#[ignore]
fn basic() {
    assert_eq!(acronym::abbreviate("Portable Network Graphics"), "PNG");
}

#[test]
#[ignore]
fn lowercase_words() {
    assert_eq!(acronym::abbreviate("Ruby on Rails"), "ROR");
}

#[test]
#[ignore]
fn camelcase() {
    assert_eq!(acronym::abbreviate("HyperText Markup Language"), "HTML");
}

#[test]
#[ignore]
fn punctuation() {
    assert_eq!(acronym::abbreviate("First In, First Out"), "FIFO");
}

#[test]
#[ignore]
fn all_caps_words() {
    assert_eq!(acronym::abbreviate("PHP: Hypertext Preprocessor"), "PHP");
}

#[test]
#[ignore]
fn non_acronym_all_caps_word() {
    assert_eq!(
        acronym::abbreviate("GNU Image Manipulation Program"),
        "GIMP"
    );
}

#[test]
#[ignore]
fn hyphenated() {
    assert_eq!(
        acronym::abbreviate("Complementary metal-oxide semiconductor"),
        "CMOS"
    );
}
