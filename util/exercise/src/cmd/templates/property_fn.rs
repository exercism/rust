/// Process a single test case for the property `{{ property }}`
///
/// All cases for the `{{ property }}` property are implemented in terms of
/// this function.
///
/// Note that you'll need to both name the expected transform which the student
/// needs to write, and name the types of the inputs and outputs.
/// While rustc _may_ be able to handle things properly given a working example,
/// students will face confusing errors if the `I` and `O` types are not
/// concrete.
fn process_{{ format_property(property=property) }}_case<I, O>(input: I, expected: O) {
    //  typical implementation:
    //  assert_eq!(
    //      student_{{ format_property(property=property) }}_func(input),
    //      expected,
    //  )
    unimplemented!()
}
