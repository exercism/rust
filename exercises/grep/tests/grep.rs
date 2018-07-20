extern crate grep;

use grep::{grep, Flags};

use std::fs;

static ILIAD_CONTENT: &'static str = "Achilles sing, O Goddess! Peleus' son;
His wrath pernicious, who ten thousand woes
Caused to Achaia's host, sent many a soul
Illustrious into Ades premature,
And Heroes gave (so stood the will of Jove)
To dogs and to all ravening fowls a prey,
When fierce dispute had separated once
The noble Chief Achilles from the son
Of Atreus, Agamemnon, King of men.
";

static MIDSUMMER_NIGHT_CONTENT: &'static str = "I do entreat your grace to pardon me.
I know not by what power I am made bold,
Nor how it may concern my modesty,
In such a presence here to plead my thoughts;
But I beseech your grace that I may know
The worst that may befall me in this case,
If I refuse to wed Demetrius.
";

static PARADISE_LOST_CONTENT: &'static str = "Of Mans First Disobedience, and the Fruit
Of that Forbidden Tree, whose mortal tast
Brought Death into the World, and all our woe,
With loss of Eden, till one greater Man
Restore us, and regain the blissful Seat,
Sing Heav'nly Muse, that on the secret top
Of Oreb, or of Sinai, didst inspire
That Shepherd, who first taught the chosen Seed
";

/// In The White Night
/// A poem by Alexander Blok(https://en.wikipedia.org/wiki/Alexander_Blok)
/// a Russian poet who is regarded as one of the most important figures of the Silver Age of Russian Poetry
/// You can read the translation here: https://lyricstranslate.com/ru/белой-ночью-месяц-красный-white-night-crimson-crescent.html
static IN_THE_WHITE_NIGHT_CONTENT: &'static str = "Белой ночью месяц красный
Выплывает в синеве.
Бродит призрачно-прекрасный,
Отражается в Неве.
Мне провидится и снится
Исполпенье тайных дум.
В вас ли доброе таится,
Красный месяц, тихий шум?..
";

struct Fixture<'a> {
    file_names: &'a [&'a str],
}

impl<'a> Fixture<'a> {
    fn new(file_names: &'a [&'a str]) -> Self {
        Fixture { file_names }
    }

    fn set_up(&self) {
        let file_name_content_pairs = self.file_names
            .iter()
            .cloned()
            .map(|file_name| {
                if file_name.ends_with("iliad.txt") {
                    (file_name, ILIAD_CONTENT)
                } else if file_name.ends_with("midsummer_night.txt") {
                    (file_name, MIDSUMMER_NIGHT_CONTENT)
                } else if file_name.ends_with("paradise_lost.txt") {
                    (file_name, PARADISE_LOST_CONTENT)
                } else {
                    (file_name, IN_THE_WHITE_NIGHT_CONTENT)
                }
            })
            .collect::<Vec<(&str, &str)>>();

        set_up_files(&file_name_content_pairs);
    }
}

impl<'a> Drop for Fixture<'a> {
    fn drop(&mut self) {
        tear_down_files(self.file_names);
    }
}

fn set_up_files(files: &[(&str, &str)]) {
    for (file_name, file_content) in files {
        fs::write(file_name, file_content).expect(&format!(
            "Error setting up file '{}' with the following content:\n{}",
            file_name, file_content
        ));
    }
}

fn tear_down_files(files: &[&str]) {
    for file_name in files {
        fs::remove_file(file_name).expect(&format!("Could not delete file '{}'", file_name));
    }
}

/// This macro is here so that every test case had it's own set of files to be used in test.
/// The approach is to create required files for every test case and to append test name to the
/// file names (so for test with a name 'test_one_file_one_match_no_flags' and a required file
/// 'iliad.txt' there would be created a file with a name
/// 'test_one_file_one_match_no_flags_iliad.txt').
/// This allows us to create files for every test case with no intersection between them.
///
/// A better way would be to create required set of files at the start of tests run and to
/// delete them after every test is finished, but there is no trivial way to create such
/// a test fixture in standard Rust, and Exercism restricts the usage of external dependencies
/// in test files. Therefore the above approach is chosen.
///
/// If you have an idea about a better way to implement test fixture for this exercise,
/// please submit PR to the Rust Exercism track: https://github.com/exercism/rust
macro_rules! set_up_test_case {
    ($(#[$flag:meta])+ $test_case_name:ident(pattern=$pattern:expr, flags=[$($grep_flag:expr),*], files=[$($file:expr),+], expected=[$($expected:expr),*])) => {
        $(#[$flag])+
        fn $test_case_name() {
            let pattern = $pattern;

            let flags = vec![$($grep_flag),*];

            let files = vec![$(concat!(stringify!($test_case_name), "_" , $file)),+];

            let expected = vec![$($expected),*];

            process_grep_case(&pattern, &flags, &files, &expected);
        }
    };
    ($(#[$flag:meta])+ $test_case_name:ident(pattern=$pattern:expr, flags=[$($grep_flag:expr),*], files=[$($file:expr),+], prefix_expected=[$($expected:expr),*])) => {
        $(#[$flag])+
        fn $test_case_name() {
            let pattern = $pattern;

            let flags = vec![$($grep_flag),*];

            let files = vec![$(concat!(stringify!($test_case_name), "_" , $file)),+];

            let expected = vec![$(concat!(stringify!($test_case_name), "_", $expected)),*];

            process_grep_case(&pattern, &flags, &files, &expected);
        }

    }
}

fn process_grep_case(pattern: &str, flags: &[&str], files: &[&str], expected: &[&str]) {
    let test_fixture = Fixture::new(files);

    test_fixture.set_up();

    let flags = Flags::new(flags);

    let grep_result = grep(pattern, &flags, files).unwrap();

    assert_eq!(grep_result, expected);
}

// Test returning a Result

#[test]
fn test_nonexistent_file_returns_error() {
    let pattern = "Agamemnon";

    let flags = Flags::new(&vec![]);

    let files = vec!["test_nonexistent_file_returns_error_iliad.txt"];

    assert!(grep(&pattern, &flags, &files).is_err());
}

#[test]
#[ignore]
fn test_grep_returns_result() {
    let pattern = "Agamemnon";

    let flags = Flags::new(&vec![]);

    let files = vec!["test_grep_returns_result_iliad.txt"];

    let test_fixture = Fixture::new(&files);

    test_fixture.set_up();

    assert!(grep(&pattern, &flags, &files).is_ok());
}

// Test grepping a single file

set_up_test_case!(#[test]
#[ignore]
test_one_file_one_match_no_flags(
    pattern = "Agamemnon",
    flags = [],
    files = ["iliad.txt"],
    expected = ["Of Atreus, Agamemnon, King of men."]
));

set_up_test_case!(#[test]
#[ignore]
test_one_file_one_match_print_line_numbers_flag(
    pattern = "Forbidden",
    flags = ["-n"],
    files = ["paradise_lost.txt"],
    expected = ["2:Of that Forbidden Tree, whose mortal tast"]
));

set_up_test_case!(#[test]
#[ignore]
test_one_file_one_match_caseinsensitive_flag(
    pattern = "FORBIDDEN",
    flags = ["-i"],
    files = ["paradise_lost.txt"],
    expected = ["Of that Forbidden Tree, whose mortal tast"]
));

set_up_test_case!(#[test]
#[ignore]
test_one_file_one_match_print_file_names_flag(
    pattern = "Forbidden",
    flags = ["-l"],
    files = ["paradise_lost.txt"],
    prefix_expected = ["paradise_lost.txt"]
));

set_up_test_case!(#[test]
#[ignore]
test_one_file_one_match_match_entire_lines_flag(
    pattern = "With loss of Eden, till one greater Man",
    flags = ["-x"],
    files = ["paradise_lost.txt"],
    expected = ["With loss of Eden, till one greater Man"]
));

set_up_test_case!(#[test]
#[ignore]
test_one_file_one_match_multiple_flags(
    pattern = "OF ATREUS, Agamemnon, KIng of MEN.",
    flags = ["-x", "-i", "-n"],
    files = ["iliad.txt"],
    expected = ["9:Of Atreus, Agamemnon, King of men."]
));

set_up_test_case!(#[test]
#[ignore]
test_one_file_several_matches_no_flags(
    pattern = "may",
    flags = [],
    files = ["midsummer_night.txt"],
    expected = [
        "Nor how it may concern my modesty,",
        "But I beseech your grace that I may know",
        "The worst that may befall me in this case,"
    ]
));

set_up_test_case!(#[test]
#[ignore]
test_one_file_several_matches_print_line_numbers_flag(
    pattern = "may",
    flags = ["-n"],
    files = ["midsummer_night.txt"],
    expected = [
        "3:Nor how it may concern my modesty,",
        "5:But I beseech your grace that I may know",
        "6:The worst that may befall me in this case,"
    ]
));

set_up_test_case!(#[test]
#[ignore]
test_one_file_several_matches_match_entire_lines_flag(
    pattern = "may",
    flags = ["-x"],
    files = ["midsummer_night.txt"],
    expected = []
));

set_up_test_case!(#[test]
#[ignore]
test_one_file_several_matches_caseinsensitive_flag(
    pattern = "ACHILLES",
    flags = ["-i"],
    files = ["iliad.txt"],
    expected = [
        "Achilles sing, O Goddess! Peleus' son;",
        "The noble Chief Achilles from the son"
    ]
));

set_up_test_case!(#[test]
#[ignore]
test_one_file_several_matches_inverted_flag(
    pattern = "Of",
    flags = ["-v"],
    files = ["paradise_lost.txt"],
    expected = [
        "Brought Death into the World, and all our woe,",
        "With loss of Eden, till one greater Man",
        "Restore us, and regain the blissful Seat,",
        "Sing Heav'nly Muse, that on the secret top",
        "That Shepherd, who first taught the chosen Seed"
    ]
));

set_up_test_case!(#[test]
#[ignore]
test_one_file_no_matches_various_flags(
    pattern = "Gandalf",
    flags = ["-n", "-l", "-x", "-i"],
    files = ["iliad.txt"],
    expected = []
));

// Test grepping multiples files at once

set_up_test_case!(#[test]
#[ignore]
test_multiple_files_one_match_no_flags(
    pattern = "Agamemnon",
    flags = [],
    files = ["iliad.txt", "midsummer_night.txt", "paradise_lost.txt"],
    prefix_expected = ["iliad.txt:Of Atreus, Agamemnon, King of men."]
));

set_up_test_case!(#[test]
#[ignore]
test_multiple_files_several_matches_no_flags(
    pattern = "may",
    flags = [],
    files = ["iliad.txt", "midsummer_night.txt", "paradise_lost.txt"],
    prefix_expected = [
        "midsummer_night.txt:Nor how it may concern my modesty,",
        "midsummer_night.txt:But I beseech your grace that I may know",
        "midsummer_night.txt:The worst that may befall me in this case,"
    ]
));

set_up_test_case!(#[test]
#[ignore]
test_multiple_files_several_matches_print_line_numbers_flag(
    pattern = "that",
    flags = ["-n"],
    files = ["iliad.txt", "midsummer_night.txt", "paradise_lost.txt"],
    prefix_expected = [
        "midsummer_night.txt:5:But I beseech your grace that I may know",
        "midsummer_night.txt:6:The worst that may befall me in this case,",
        "paradise_lost.txt:2:Of that Forbidden Tree, whose mortal tast",
        "paradise_lost.txt:6:Sing Heav'nly Muse, that on the secret top"
    ]
));

set_up_test_case!(#[test]
#[ignore]
test_multiple_files_one_match_print_file_names_flag(
    pattern = "who",
    flags = ["-l"],
    files = ["iliad.txt", "midsummer_night.txt", "paradise_lost.txt"],
    prefix_expected = ["iliad.txt", "paradise_lost.txt"]
));

set_up_test_case!(#[test]
#[ignore]
test_multiple_files_several_matches_caseinsensitive_flag(
    pattern = "TO",
    flags = ["-i"],
    files = ["iliad.txt", "midsummer_night.txt", "paradise_lost.txt"],
    prefix_expected = [
        "iliad.txt:Caused to Achaia's host, sent many a soul",
        "iliad.txt:Illustrious into Ades premature,",
        "iliad.txt:And Heroes gave (so stood the will of Jove)",
        "iliad.txt:To dogs and to all ravening fowls a prey,",
        "midsummer_night.txt:I do entreat your grace to pardon me.",
        "midsummer_night.txt:In such a presence here to plead my thoughts;",
        "midsummer_night.txt:If I refuse to wed Demetrius.",
        "paradise_lost.txt:Brought Death into the World, and all our woe,",
        "paradise_lost.txt:Restore us, and regain the blissful Seat,",
        "paradise_lost.txt:Sing Heav'nly Muse, that on the secret top"
    ]
));

set_up_test_case!(
    #[test]
    #[ignore]
    test_multiple_files_several_matches_caseinsensitive_flag_utf8(
        pattern = "В", // This letter stands for cyrillic 'Ve' and not latin 'B'. Therefore there should be no matches from paradise_lost.txt
        flags = ["-i"],
        files = ["paradise_lost.txt", "in_the_white_night.txt"],
        prefix_expected = [
            "in_the_white_night.txt:Выплывает в синеве.",
            "in_the_white_night.txt:Отражается в Неве.",
            "in_the_white_night.txt:Мне провидится и снится",
            "in_the_white_night.txt:В вас ли доброе таится,"
        ]
    )
);

set_up_test_case!(#[test]
#[ignore]
test_multiple_files_several_matches_inverted_flag(
    pattern = "a",
    flags = ["-v"],
    files = ["iliad.txt", "midsummer_night.txt", "paradise_lost.txt"],
    prefix_expected = [
        "iliad.txt:Achilles sing, O Goddess! Peleus' son;",
        "iliad.txt:The noble Chief Achilles from the son",
        "midsummer_night.txt:If I refuse to wed Demetrius."
    ]
));

set_up_test_case!(#[test]
#[ignore]
test_multiple_files_one_match_match_entire_lines_flag(
    pattern = "But I beseech your grace that I may know",
    flags = ["-x"],
    files = ["iliad.txt", "midsummer_night.txt", "paradise_lost.txt"],
    prefix_expected = ["midsummer_night.txt:But I beseech your grace that I may know"]
));

set_up_test_case!(#[test]
#[ignore]
test_multiple_files_one_match_multiple_flags(
    pattern = "WITH LOSS OF EDEN, TILL ONE GREATER MAN",
    flags = ["-n", "-i", "-x"],
    files = ["iliad.txt", "midsummer_night.txt", "paradise_lost.txt"],
    prefix_expected = ["paradise_lost.txt:4:With loss of Eden, till one greater Man"]
));

set_up_test_case!(#[test]
#[ignore]
test_multiple_files_no_matches_various_flags(
    pattern = "Frodo",
    flags = ["-n", "-i", "-x", "-l"],
    files = ["iliad.txt", "midsummer_night.txt", "paradise_lost.txt"],
    expected = []
));
