use atbash_cipher::*;

#[test]
fn encode_yes() {
    let output = encode("yes");
    let expected = "bvh";

    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn encode_no() {
    let output = encode("no");
    let expected = "ml";

    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn encode_omg() {
    let output = encode("OMG");
    let expected = "lnt";

    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn encode_spaces() {
    let output = encode("O M G");
    let expected = "lnt";

    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn encode_mindblowingly() {
    let output = encode("mindblowingly");
    let expected = "nrmwy oldrm tob";

    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn encode_numbers() {
    let output = encode("Testing,1 2 3, testing.");
    let expected = "gvhgr mt123 gvhgr mt";

    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn encode_deep_thought() {
    let output = encode("Truth is fiction.");
    let expected = "gifgs rhurx grlm";

    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn encode_all_the_letters() {
    let output = encode("The quick brown fox jumps over the lazy dog.");
    let expected = "gsvjf rxpyi ldmul cqfnk hlevi gsvoz abwlt";

    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn decode_exercism() {
    let output = decode("vcvix rhn");
    let expected = "exercism";

    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn decode_a_sentence() {
    let output = decode("zmlyh gzxov rhlug vmzhg vkkrm thglm v");
    let expected = "anobstacleisoftenasteppingstone";

    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn decode_numbers() {
    let output = decode("gvhgr mt123 gvhgr mt");
    let expected = "testing123testing";

    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn decode_all_the_letters() {
    let output = decode("gsvjf rxpyi ldmul cqfnk hlevi gsvoz abwlt");
    let expected = "thequickbrownfoxjumpsoverthelazydog";

    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn decode_with_too_many_spaces() {
    let output = decode("vc vix    r hn");
    let expected = "exercism";

    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn decode_with_no_spaces() {
    let output = decode("zmlyhgzxovrhlugvmzhgvkkrmthglmv");
    let expected = "anobstacleisoftenasteppingstone";

    assert_eq!(output, expected);
}
