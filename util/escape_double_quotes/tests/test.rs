use escape_double_quotes::utils::escape_double_quotes::escape_double_quotes;

#[test]
fn test_no_double_quotes() {
    let input = "let x = 5;";
    let expected = "let x = 5;";
    assert_eq!(escape_double_quotes(input), expected);
}

#[test]
fn test_simple_double_quotes() {
    let input = "let something = \"string\";";
    let expected = "let something = \\\"string\\\";";
    assert_eq!(escape_double_quotes(input), expected);
}

#[test]
fn test_braces_with_double_quotes() {
    let input = "let expected = \"${expected | join(\\\"\\n\\\")}$\";";
    let expected = "let expected = \\\"${expected | join(\\\"\\n\\\")}$\\\";";
    assert_eq!(escape_double_quotes(input), expected);
}

#[test]
fn test_mixed_double_quotes() {
    let input = "let a = \"value\"; let b = \"${value | filter(\\\"text\\\")}$\";";
    let expected = "let a = \\\"value\\\"; let b = \\\"${value | filter(\\\"text\\\")}$\\\";";
    assert_eq!(escape_double_quotes(input), expected);
}

#[test]
fn test_nested_braces() {
    let input = "let nested = \"${outer {inner | escape(\\\"\\n\\\")}}$\";";
    let expected = "let nested = \\\"${outer {inner | escape(\\\"\\n\\\")}}$\\\";";
    assert_eq!(escape_double_quotes(input), expected);
}
