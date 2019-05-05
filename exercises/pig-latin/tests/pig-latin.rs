use pig_latin as pl;

#[test]
fn test_word_beginning_with_a() {
    assert_eq!("appleay", pl::translate("apple"));
}

#[test]
#[ignore]
fn test_word_beginning_with_e() {
    assert_eq!("earay", pl::translate("ear"));
}

#[test]
#[ignore]
fn test_word_beginning_with_i() {
    assert_eq!("iglooay", pl::translate("igloo"));
}

#[test]
#[ignore]
fn test_word_beginning_with_o() {
    assert_eq!("objectay", pl::translate("object"));
}

#[test]
#[ignore]
fn test_word_beginning_with_u() {
    assert_eq!("underay", pl::translate("under"));
}

#[test]
#[ignore]
fn test_word_beginning_with_a_vowel_and_followed_by_a_qu() {
    assert_eq!("equalay", pl::translate("equal"));
}

#[test]
#[ignore]
fn test_word_beginning_with_p() {
    assert_eq!("igpay", pl::translate("pig"));
}

#[test]
#[ignore]
fn test_word_beginning_with_k() {
    assert_eq!("oalakay", pl::translate("koala"));
}

#[test]
#[ignore]
fn test_word_beginning_with_y() {
    assert_eq!("ellowyay", pl::translate("yellow"));
}

#[test]
#[ignore]
fn test_word_beginning_with_x() {
    assert_eq!("enonxay", pl::translate("xenon"));
}

#[test]
#[ignore]
fn test_word_beginning_with_q_without_a_following_u() {
    assert_eq!("atqay", pl::translate("qat"));
}

#[test]
#[ignore]
fn test_word_beginning_with_ch() {
    assert_eq!("airchay", pl::translate("chair"));
}

#[test]
#[ignore]
fn test_word_beginning_with_qu() {
    assert_eq!("eenquay", pl::translate("queen"));
}

#[test]
#[ignore]
fn test_word_beginning_with_qu_and_a_preceding_consonant() {
    assert_eq!("aresquay", pl::translate("square"));
}

#[test]
#[ignore]
fn test_word_beginning_with_th() {
    assert_eq!("erapythay", pl::translate("therapy"));
}

#[test]
#[ignore]
fn test_word_beginning_with_thr() {
    assert_eq!("ushthray", pl::translate("thrush"));
}

#[test]
#[ignore]
fn test_word_beginning_with_sch() {
    assert_eq!("oolschay", pl::translate("school"));
}

#[test]
#[ignore]
fn test_word_beginning_with_yt() {
    assert_eq!("yttriaay", pl::translate("yttria"));
}

#[test]
#[ignore]
fn test_word_beginning_with_xr() {
    assert_eq!("xrayay", pl::translate("xray"));
}

#[test]
#[ignore]
fn test_y_is_treated_like_a_vowel_at_the_end_of_a_consonant_cluster() {
    assert_eq!("ythmrhay", pl::translate("rhythm"));
}

#[test]
#[ignore]
fn test_a_whole_phrase() {
    assert_eq!("ickquay astfay unray", pl::translate("quick fast run"));
}
