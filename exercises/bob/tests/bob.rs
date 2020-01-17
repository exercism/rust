

//NEW
#[test]
#[ignore]
/// ending with whitespace
fn test_ending_with_whitespace() {
process_response_case({let mut hm = ::std::collections::HashMap::new(); hm.insert("heyBob", "Okay if like my  spacebar  quite a bit?   "); hm}, "Sure.");
}


//NEW
#[test]
#[ignore]
/// shouting numbers
fn test_shouting_numbers() {
process_response_case({let mut hm = ::std::collections::HashMap::new(); hm.insert("heyBob", "1, 2, 3 GO!"); hm}, "Whoa, chill out!");
}


//NEW
#[test]
#[ignore]
/// other whitespace
fn test_other_whitespace() {
process_response_case({let mut hm = ::std::collections::HashMap::new(); hm.insert("heyBob", "\r\r 	"); hm}, "Fine. Be that way!");
}


//NEW
#[test]
#[ignore]
/// shouting with special characters
fn test_shouting_with_special_characters() {
process_response_case({let mut hm = ::std::collections::HashMap::new(); hm.insert("heyBob", "ZOMG THE %^*@#$(*^ ZOMBIES ARE COMING!!11!!1!"); hm}, "Whoa, chill out!");
}


//NEW
#[test]
#[ignore]
/// talking forcefully
fn test_talking_forcefully() {
process_response_case({let mut hm = ::std::collections::HashMap::new(); hm.insert("heyBob", "Hi there!"); hm}, "Whatever.");
}


//NEW
#[test]
#[ignore]
/// prattling on
fn test_prattling_on() {
process_response_case({let mut hm = ::std::collections::HashMap::new(); hm.insert("heyBob", "Wait! Hang on. Are you going to be OK?"); hm}, "Sure.");
}


//NEW
#[test]
#[ignore]
/// forceful question
fn test_forceful_question() {
process_response_case({let mut hm = ::std::collections::HashMap::new(); hm.insert("heyBob", "WHAT'S GOING ON?"); hm}, "Calm down, I know what I'm doing!");
}


//NEW
#[test]
#[ignore]
/// shouting with no exclamation mark
fn test_shouting_with_no_exclamation_mark() {
process_response_case({let mut hm = ::std::collections::HashMap::new(); hm.insert("heyBob", "I HATE THE DENTIST"); hm}, "Whoa, chill out!");
}


//NEW
#[test]
#[ignore]
/// asking gibberish
fn test_asking_gibberish() {
process_response_case({let mut hm = ::std::collections::HashMap::new(); hm.insert("heyBob", "fffbbcbeab?"); hm}, "Sure.");
}


//NEW
#[test]
#[ignore]
/// question with no letters
fn test_question_with_no_letters() {
process_response_case({let mut hm = ::std::collections::HashMap::new(); hm.insert("heyBob", "4?"); hm}, "Sure.");
}


//NEW
#[test]
#[ignore]
/// no letters
fn test_no_letters() {
process_response_case({let mut hm = ::std::collections::HashMap::new(); hm.insert("heyBob", "1, 2, 3"); hm}, "Whatever.");
}


//NEW
#[test]
#[ignore]
/// statement containing question mark
fn test_statement_containing_question_mark() {
process_response_case({let mut hm = ::std::collections::HashMap::new(); hm.insert("heyBob", "Ending with ? means a question."); hm}, "Whatever.");
}


//NEW
#[test]
#[ignore]
/// multiple line question
fn test_multiple_line_question() {
process_response_case({let mut hm = ::std::collections::HashMap::new(); hm.insert("heyBob", "
Does this cryogenic chamber make me look fat?
No."); hm}, "Whatever.");
}


//NEW
#[test]
#[ignore]
/// non-question ending with whitespace
fn test_nonquestion_ending_with_whitespace() {
process_response_case({let mut hm = ::std::collections::HashMap::new(); hm.insert("heyBob", "This is a statement ending with whitespace      "); hm}, "Whatever.");
}


//NEW
#[test]
#[ignore]
/// shouting
fn test_shouting() {
process_response_case({let mut hm = ::std::collections::HashMap::new(); hm.insert("heyBob", "WATCH OUT!"); hm}, "Whoa, chill out!");
}


//NEW
#[test]
#[ignore]
/// non-letters with question
fn test_nonletters_with_question() {
process_response_case({let mut hm = ::std::collections::HashMap::new(); hm.insert("heyBob", ":) ?"); hm}, "Sure.");
}


//NEW
#[test]
#[ignore]
/// stating something
fn test_stating_something() {
process_response_case({let mut hm = ::std::collections::HashMap::new(); hm.insert("heyBob", "Tom-ay-to, tom-aaaah-to."); hm}, "Whatever.");
}


//NEW
#[test]
#[ignore]
/// shouting gibberish
fn test_shouting_gibberish() {
process_response_case({let mut hm = ::std::collections::HashMap::new(); hm.insert("heyBob", "FCECDFCAAB"); hm}, "Whoa, chill out!");
}


//NEW
#[test]
#[ignore]
/// asking a question
fn test_asking_a_question() {
process_response_case({let mut hm = ::std::collections::HashMap::new(); hm.insert("heyBob", "Does this cryogenic chamber make me look fat?"); hm}, "Sure.");
}


//NEW
/// Process a single test case for the property `response`
///
/// All cases for the `response` property are implemented
/// in terms of this function.
/// 
/// Note that you'll need to both name the expected transform which
/// the student needs to write, and name the types of the inputs and outputs.
/// While rustc _may_ be able to handle things properly given a working example,
/// students will face confusing errors if the `I` and `O` types are not concrete.
/// 
fn process_response_case<I, O>(input: I, expected: O) {
//  typical implementation:
//  assert_eq!(
//      student_response_func(input),
//      expected
//  )
    unimplemented!()
}


//NEW
#[test]
#[ignore]
/// asking a numeric question
fn test_asking_a_numeric_question() {
process_response_case({let mut hm = ::std::collections::HashMap::new(); hm.insert("heyBob", "You are, what, like 15?"); hm}, "Sure.");
}


//NEW
#[test]
#[ignore]
/// silence
fn test_silence() {
process_response_case({let mut hm = ::std::collections::HashMap::new(); hm.insert("heyBob", ""); hm}, "Fine. Be that way!");
}


//NEW
#[test]
#[ignore]
/// starting with whitespace
fn test_starting_with_whitespace() {
process_response_case({let mut hm = ::std::collections::HashMap::new(); hm.insert("heyBob", "         hmmmmmmm..."); hm}, "Whatever.");
}


//NEW
#[test]
#[ignore]
/// using acronyms in regular speech
fn test_using_acronyms_in_regular_speech() {
process_response_case({let mut hm = ::std::collections::HashMap::new(); hm.insert("heyBob", "It's OK if you don't want to go work for NASA."); hm}, "Whatever.");
}


//NEW
#[test]
#[ignore]
/// alternate silence
fn test_alternate_silence() {
process_response_case({let mut hm = ::std::collections::HashMap::new(); hm.insert("heyBob", "										"); hm}, "Fine. Be that way!");
}


//NEW
#[test]
#[ignore]
/// prolonged silence
fn test_prolonged_silence() {
process_response_case({let mut hm = ::std::collections::HashMap::new(); hm.insert("heyBob", "          "); hm}, "Fine. Be that way!");
}

