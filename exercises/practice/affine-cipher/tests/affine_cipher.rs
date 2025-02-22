use affine_cipher::*;

use affine_cipher::AffineCipherError::NotCoprime;

#[test]
fn encode_yes() {
    let phrase = "yes";
    let (a, b) = (5, 7);
    let output = encode(phrase, a, b);
    let expected = Ok("xbt".into());
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn encode_no() {
    let phrase = "no";
    let (a, b) = (15, 18);
    let output = encode(phrase, a, b);
    let expected = Ok("fu".into());
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn encode_omg() {
    let phrase = "OMG";
    let (a, b) = (21, 3);
    let output = encode(phrase, a, b);
    let expected = Ok("lvz".into());
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn encode_o_m_g() {
    let phrase = "O M G";
    let (a, b) = (25, 47);
    let output = encode(phrase, a, b);
    let expected = Ok("hjp".into());
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn encode_mindblowingly() {
    let phrase = "mindblowingly";
    let (a, b) = (11, 15);
    let output = encode(phrase, a, b);
    let expected = Ok("rzcwa gnxzc dgt".into());
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn encode_numbers() {
    let phrase = "Testing,1 2 3, testing.";
    let (a, b) = (3, 4);
    let output = encode(phrase, a, b);
    let expected = Ok("jqgjc rw123 jqgjc rw".into());
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn encode_deep_thought() {
    let phrase = "Truth is fiction.";
    let (a, b) = (5, 17);
    let output = encode(phrase, a, b);
    let expected = Ok("iynia fdqfb ifje".into());
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn encode_all_the_letters() {
    let phrase = "The quick brown fox jumps over the lazy dog.";
    let (a, b) = (17, 33);
    let output = encode(phrase, a, b);
    let expected = Ok("swxtj npvyk lruol iejdc blaxk swxmh qzglf".into());
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn encode_with_a_not_coprime_to_m() {
    let phrase = "This is a test.";
    let (a, b) = (6, 17);
    let output = encode(phrase, a, b);
    let expected = Err(NotCoprime(6));
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn decode_exercism() {
    let phrase = "tytgn fjr";
    let (a, b) = (3, 7);
    let output = decode(phrase, a, b);
    let expected = Ok("exercism".into());
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn decode_a_sentence() {
    let phrase = "qdwju nqcro muwhn odqun oppmd aunwd o";
    let (a, b) = (19, 16);
    let output = decode(phrase, a, b);
    let expected = Ok("anobstacleisoftenasteppingstone".into());
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn decode_numbers() {
    let phrase = "odpoz ub123 odpoz ub";
    let (a, b) = (25, 7);
    let output = decode(phrase, a, b);
    let expected = Ok("testing123testing".into());
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn decode_all_the_letters() {
    let phrase = "swxtj npvyk lruol iejdc blaxk swxmh qzglf";
    let (a, b) = (17, 33);
    let output = decode(phrase, a, b);
    let expected = Ok("thequickbrownfoxjumpsoverthelazydog".into());
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn decode_with_no_spaces_in_input() {
    let phrase = "swxtjnpvyklruoliejdcblaxkswxmhqzglf";
    let (a, b) = (17, 33);
    let output = decode(phrase, a, b);
    let expected = Ok("thequickbrownfoxjumpsoverthelazydog".into());
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn decode_with_too_many_spaces() {
    let phrase = "vszzm    cly   yd cg    qdp";
    let (a, b) = (15, 16);
    let output = decode(phrase, a, b);
    let expected = Ok("jollygreengiant".into());
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn decode_with_a_not_coprime_to_m() {
    let phrase = "Test";
    let (a, b) = (13, 5);
    let output = decode(phrase, a, b);
    let expected = Err(NotCoprime(13));
    assert_eq!(output, expected);
}
