use atbash_cipher as cipher;

#[test]
fn test_encode_yes() {
    assert_eq!("bvh", cipher::encode("yes"));
}

#[test]
#[ignore]
fn test_encode_no() {
    assert_eq!("ml", cipher::encode("no"));
}

#[test]
#[ignore]
fn test_encode_omg() {
    assert_eq!("lnt", cipher::encode("OMG"));
}

#[test]
#[ignore]
fn test_encode_spaces() {
    assert_eq!("lnt", cipher::encode("O M G"));
}

#[test]
#[ignore]
fn test_encode_mindblowingly() {
    assert_eq!("nrmwy oldrm tob", cipher::encode("mindblowingly"));
}

#[test]
#[ignore]
fn test_encode_numbers() {
    assert_eq!(
        "gvhgr mt123 gvhgr mt",
        cipher::encode("Testing,1 2 3, testing.")
    );
}

#[test]
#[ignore]
fn test_encode_deep_thought() {
    assert_eq!("gifgs rhurx grlm", cipher::encode("Truth is fiction."));
}

#[test]
#[ignore]
fn test_encode_all_the_letters() {
    assert_eq!(
        "gsvjf rxpyi ldmul cqfnk hlevi gsvoz abwlt",
        cipher::encode("The quick brown fox jumps over the lazy dog.")
    );
}

#[test]
#[ignore]
fn test_encode_ignores_non_ascii() {
    assert_eq!("mlmzh xrrrt mlivw", cipher::encode("non ascii éignored"));
}

#[test]
#[ignore]
fn test_decode_exercism() {
    assert_eq!("exercism", cipher::decode("vcvix rhn"));
}

#[test]
#[ignore]
fn test_decode_a_sentence() {
    assert_eq!(
        "anobstacleisoftenasteppingstone",
        cipher::decode("zmlyh gzxov rhlug vmzhg vkkrm thglm v")
    );
}

#[test]
#[ignore]
fn test_decode_numbers() {
    assert_eq!("testing123testing", cipher::decode("gvhgr mt123 gvhgr mt"));
}

#[test]
#[ignore]
fn test_decode_all_the_letters() {
    assert_eq!(
        "thequickbrownfoxjumpsoverthelazydog",
        cipher::decode("gsvjf rxpyi ldmul cqfnk hlevi gsvoz abwlt")
    );
}
