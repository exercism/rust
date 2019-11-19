use affine_cipher::*;

#[test]
fn encode_yes() {
    assert_eq!(encode("yes", 5, 7).unwrap(), "xbt")
}

#[test]
#[ignore]
fn encode_no() {
    assert_eq!(encode("no", 15, 18).unwrap(), "fu")
}

#[test]
#[ignore]
fn encode_omg() {
    assert_eq!(encode("OMG", 21, 3).unwrap(), "lvz")
}

#[test]
#[ignore]
fn encode_o_m_g() {
    assert_eq!(encode("O M G", 25, 47).unwrap(), "hjp")
}

#[test]
#[ignore]
fn encode_mindblowingly() {
    assert_eq!(encode("mindblowingly", 11, 15).unwrap(), "rzcwa gnxzc dgt")
}

#[test]
#[ignore]
fn encode_numbers() {
    assert_eq!(
        encode("Testing,1 2 3, testing.", 3, 4).unwrap(),
        "jqgjc rw123 jqgjc rw"
    )
}

#[test]
#[ignore]
fn encode_deep_thought() {
    assert_eq!(
        encode("Truth is fiction", 5, 17).unwrap(),
        "iynia fdqfb ifje"
    )
}

#[test]
#[ignore]
fn encode_all_the_letters() {
    assert_eq!(
        encode("The quick brown fox jumps over the lazy dog.", 17, 33).unwrap(),
        "swxtj npvyk lruol iejdc blaxk swxmh qzglf"
    )
}

#[test]
#[ignore]
fn encode_with_a_not_coprime_to_m() {
    const EXPECTED_ERROR: AffineCipherError = AffineCipherError::NotCoprime(6);
    match encode("This is a test.", 6, 17) {
        Err(EXPECTED_ERROR) => (),
        Err(err) => panic!(
            "Incorrect error: expected: {:?}, actual: {:?}.",
            EXPECTED_ERROR, err
        ),
        Ok(r) => panic!(
            "Cannot encode/decode when a is coprime to m: expected: {:?}, actual: {:?}.",
            EXPECTED_ERROR, r
        ),
    }
}

#[test]
#[ignore]
fn decode_exercism() {
    assert_eq!(decode("tytgn fjr", 3, 7).unwrap(), "exercism")
}

#[test]
#[ignore]
fn decode_a_sentence() {
    assert_eq!(
        encode("anobstacleisoftenasteppingstone", 19, 16).unwrap(),
        "qdwju nqcro muwhn odqun oppmd aunwd o"
    );
    assert_eq!(
        decode("qdwju nqcro muwhn odqun oppmd aunwd o", 19, 16).unwrap(),
        "anobstacleisoftenasteppingstone"
    )
}

#[test]
#[ignore]
fn decode_numbers() {
    assert_eq!(
        decode("odpoz ub123 odpoz ub", 25, 7).unwrap(),
        "testing123testing"
    )
}

#[test]
#[ignore]
fn decode_all_the_letters() {
    assert_eq!(
        decode("swxtj npvyk lruol iejdc blaxk swxmh qzglf", 17, 33).unwrap(),
        "thequickbrownfoxjumpsoverthelazydog"
    )
}

#[test]
#[ignore]
fn decode_with_no_spaces_in_input() {
    assert_eq!(
        decode("swxtjnpvyklruoliejdcblaxkswxmhqzglf", 17, 33).unwrap(),
        "thequickbrownfoxjumpsoverthelazydog"
    )
}

#[test]
#[ignore]
fn decode_with_too_many_spaces() {
    assert_eq!(
        decode("vszzm    cly   yd cg    qdp", 15, 16).unwrap(),
        "jollygreengiant"
    )
}

#[test]
#[ignore]
fn decode_with_a_not_coprime_to_m() {
    const EXPECTED_ERROR: AffineCipherError = AffineCipherError::NotCoprime(13);
    match decode("Test", 13, 11) {
        Err(EXPECTED_ERROR) => (),
        Err(e) => panic!(
            "Incorrect error: expected: {:?}, actual: {:?}.",
            EXPECTED_ERROR, e
        ),
        Ok(r) => panic!(
            "Cannot encode/decode when a is coprime to m: expected: {:?}, actual: {:?}.",
            EXPECTED_ERROR, r
        ),
    }
}
