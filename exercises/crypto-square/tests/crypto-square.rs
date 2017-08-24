extern crate crypto_square;
use crypto_square::encrypt;

fn test(input: &str, output: &str) {
    assert_eq!(&encrypt(input), output);
}

#[test]
fn test_empty_input() {
    test("", "")
}


#[test]
fn test_encrypt_also_decrypts_square() {
    // note that you only get the exact input back if:
    // 1. no punctuation
    // 2. even spacing
    // 3. all lowercase
    // 4. square input
    let example = "lime anda coco anut";
    assert_eq!(example, &encrypt(&encrypt(example)));
}

#[test]
fn test_example() {
    test(
        "If man was meant to stay on the ground, god would have given us roots.",
        "imtgdvs fearwer mayoogo anouuio ntnnlvt wttddes aohghn sseoau",
    )
}

#[test]
fn test_empty_last_line() {
    test("congratulate", "crl oaa ntt gue")
}

#[test]
fn test_spaces_are_reorganized() {
    test("abet", "ae bt");
    test("a bet", "ae bt");
    test("     a  b     e      t             ", "ae bt");
}

#[test]
fn test_everything_becomes_lowercase() {
    test("caSe", "cs ae");
    test("cAsE", "cs ae");
    test("CASE", "cs ae");
}

#[test]
fn test_ignore_non_ascii_chars() {
    test(
        "She got her education, then got rich programming: 👩‍🎓🎓👩‍💻💰",
        "setnhm hrigpm eeoori gdnton outrgg tchir haeca",
    );
}

#[test]
fn test_long() {
    test(
        r#"
We choose to go to the moon.

We choose to go to the moon in this decade and do the other things,
not because they are easy, but because they are hard, because that
goal will serve to organize and measure the best of our energies and
skills, because that challenge is one that we are willing to accept,
one we are unwilling to postpone, and one which we intend to win,
and the others, too.

-- John F. Kennedy, 12 September 1962
        "#,
        &(String::from("wohaseagabnttenwan eoennaronedcwpghnf cnmdoseaisshettidk ") +
              "hwodtyhlztkaaoocte oeoobbaweoilrnphhn ocnteuriaflleeowen shihctdlnolewwseoe " +
              "eoneabbldusnietitd totoueesmrbglapnhy oshtscceeeeelrotes geiheaaranciienere " +
              "otsetuuvseasnuensp todrhsseuruognadtt ogeteeetrgsntwntoe tochyttoeieeoidoom " +
              "htaiahhotettalowjb eodnrearhshhclnioe mtegeytgeaaacienhr"),
    )
}
