use rail_fence_cipher::RailFence;

#[test]
fn encode_with_two_rails() {
    let input = "XOXOXOXOXOXOXOXOXO";
    let rails = 2;
    let rail_fence = RailFence::new(rails);
    let output = rail_fence.encode(input);
    let expected = "XXXXXXXXXOOOOOOOOO";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn encode_with_three_rails() {
    let input = "WEAREDISCOVEREDFLEEATONCE";
    let rails = 3;
    let rail_fence = RailFence::new(rails);
    let output = rail_fence.encode(input);
    let expected = "WECRLTEERDSOEEFEAOCAIVDEN";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn encode_with_ending_in_the_middle() {
    let input = "EXERCISES";
    let rails = 4;
    let rail_fence = RailFence::new(rails);
    let output = rail_fence.encode(input);
    let expected = "ESXIEECSR";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn decode_with_three_rails() {
    let input = "TEITELHDVLSNHDTISEIIEA";
    let rails = 3;
    let rail_fence = RailFence::new(rails);
    let output = rail_fence.decode(input);
    let expected = "THEDEVILISINTHEDETAILS";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn decode_with_five_rails() {
    let input = "EIEXMSMESAORIWSCE";
    let rails = 5;
    let rail_fence = RailFence::new(rails);
    let output = rail_fence.decode(input);
    let expected = "EXERCISMISAWESOME";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn decode_with_six_rails() {
    let input = "133714114238148966225439541018335470986172518171757571896261";
    let rails = 6;
    let rail_fence = RailFence::new(rails);
    let output = rail_fence.decode(input);
    let expected = "112358132134558914423337761098715972584418167651094617711286";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn encode_wide_characters() {
    let input = "古池蛙飛び込む水の音";
    let rails = 3;
    let rail_fence = RailFence::new(rails);
    let output = rail_fence.encode(input);
    let expected = "古びの池飛込水音蛙む";
    assert_eq!(output, expected);
}
