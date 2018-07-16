extern crate grep;

use grep::grep;

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

fn process_grep_case(pattern: &str, flags: &[&str], files: &[&str], expected: &[&str]) {
    let test_fixture = Fixture::new(files);

    test_fixture.set_up();

    let grep_result = grep(pattern, flags, files);

    assert_eq!(grep_result, expected);
}

// Test grepping a single file

#[test]
/// One file, one match, no flags
fn test_one_file_one_match_no_flags() {
    let pattern = "Agamemnon";

    let flags = vec![];

    let files = vec!["test_one_file_one_match_no_flags_iliad.txt"];

    let expected = vec!["Of Atreus, Agamemnon, King of men."];

    process_grep_case(&pattern, &flags, &files, &expected);
}

#[test]
#[ignore]
/// One file, one match, print line numbers flag
fn test_one_file_one_match_print_line_numbers_flag() {
    let pattern = "Forbidden";

    let flags = vec!["-n"];

    let files = vec!["test_one_file_one_match_print_line_numbers_flag_paradise_lost.txt"];

    let expected = vec!["2:Of that Forbidden Tree, whose mortal tast"];

    process_grep_case(&pattern, &flags, &files, &expected);
}

#[test]
#[ignore]
/// One file, one match, case-insensitive flag
fn test_one_file_one_match_caseinsensitive_flag() {
    let pattern = "FORBIDDEN";

    let flags = vec!["-i"];

    let files = vec!["test_one_file_one_match_caseinsensitive_flag_paradise_lost.txt"];

    let expected = vec!["Of that Forbidden Tree, whose mortal tast"];

    process_grep_case(&pattern, &flags, &files, &expected);
}

#[test]
#[ignore]
/// One file, one match, print file names flag
fn test_one_file_one_match_print_file_names_flag() {
    let pattern = "Forbidden";

    let flags = vec!["-l"];

    let files = vec!["test_one_file_one_match_print_file_names_flag_paradise_lost.txt"];

    let expected = vec!["test_one_file_one_match_print_file_names_flag_paradise_lost.txt"];

    process_grep_case(&pattern, &flags, &files, &expected);
}

#[test]
#[ignore]
/// One file, one match, match entire lines flag
fn test_one_file_one_match_match_entire_lines_flag() {
    let pattern = "With loss of Eden, till one greater Man";

    let flags = vec!["-x"];

    let files = vec!["test_one_file_one_match_match_entire_lines_flag_paradise_lost.txt"];

    let expected = vec!["With loss of Eden, till one greater Man"];

    process_grep_case(&pattern, &flags, &files, &expected);
}

#[test]
#[ignore]
/// One file, one match, multiple flags
fn test_one_file_one_match_multiple_flags() {
    let pattern = "OF ATREUS, Agamemnon, KIng of MEN.";

    let flags = vec!["-x", "-i", "-n"];

    let files = vec!["test_one_file_one_match_multiple_flags_iliad.txt"];

    let expected = vec!["9:Of Atreus, Agamemnon, King of men."];

    process_grep_case(&pattern, &flags, &files, &expected);
}

#[test]
#[ignore]
/// One file, several matches, no flags
fn test_one_file_several_matches_no_flags() {
    let pattern = "may";

    let flags = vec![];

    let files = vec!["test_one_file_several_matches_no_flags_midsummer_night.txt"];

    let expected = vec![
        "Nor how it may concern my modesty,",
        "But I beseech your grace that I may know",
        "The worst that may befall me in this case,",
    ];

    process_grep_case(&pattern, &flags, &files, &expected);
}

#[test]
#[ignore]
/// One file, several matches, print line numbers flag
fn test_one_file_several_matches_print_line_numbers_flag() {
    let pattern = "may";

    let flags = vec!["-n"];

    let files = vec!["test_one_file_several_matches_print_line_numbers_flag_midsummer_night.txt"];

    let expected = vec![
        "3:Nor how it may concern my modesty,",
        "5:But I beseech your grace that I may know",
        "6:The worst that may befall me in this case,",
    ];

    process_grep_case(&pattern, &flags, &files, &expected);
}

#[test]
#[ignore]
/// One file, several matches, match entire lines flag
fn test_one_file_several_matches_match_entire_lines_flag() {
    let pattern = "may";

    let flags = vec!["-x"];

    let files = vec!["test_one_file_several_matches_match_entire_lines_flag_midsummer_night.txt"];

    let expected = vec![];

    process_grep_case(&pattern, &flags, &files, &expected);
}

#[test]
#[ignore]
/// One file, several matches, case-insensitive flag
fn test_one_file_several_matches_caseinsensitive_flag() {
    let pattern = "ACHILLES";

    let flags = vec!["-i"];

    let files = vec!["test_one_file_several_matches_caseinsensitive_flag_iliad.txt"];

    let expected = vec![
        "Achilles sing, O Goddess! Peleus' son;",
        "The noble Chief Achilles from the son",
    ];

    process_grep_case(&pattern, &flags, &files, &expected);
}

#[test]
#[ignore]
/// One file, several matches, inverted flag
fn test_one_file_several_matches_inverted_flag() {
    let pattern = "Of";

    let flags = vec!["-v"];

    let files = vec!["test_one_file_several_matches_inverted_flag_paradise_lost.txt"];

    let expected = vec![
        "Brought Death into the World, and all our woe,",
        "With loss of Eden, till one greater Man",
        "Restore us, and regain the blissful Seat,",
        "Sing Heav'nly Muse, that on the secret top",
        "That Shepherd, who first taught the chosen Seed",
    ];

    process_grep_case(&pattern, &flags, &files, &expected);
}

#[test]
#[ignore]
/// One file, no matches, various flags
fn test_one_file_no_matches_various_flags() {
    let pattern = "Gandalf";

    let flags = vec!["-n", "-l", "-x", "-i"];

    let files = vec!["test_one_file_no_matches_various_flags_iliad.txt"];

    let expected = vec![];

    process_grep_case(&pattern, &flags, &files, &expected);
}

// Test grepping multiples files at once

#[test]
#[ignore]
/// Multiple files, one match, no flags
fn test_multiple_files_one_match_no_flags() {
    let pattern = "Agamemnon";

    let flags = vec![];

    let files = vec![
        "test_multiple_files_one_match_no_flags_iliad.txt",
        "test_multiple_files_one_match_no_flags_midsummer_night.txt",
        "test_multiple_files_one_match_no_flags_paradise_lost.txt",
    ];

    let expected =
        vec!["test_multiple_files_one_match_no_flags_iliad.txt:Of Atreus, Agamemnon, King of men."];

    process_grep_case(&pattern, &flags, &files, &expected);
}

#[test]
#[ignore]
/// Multiple files, several matches, no flags
fn test_multiple_files_several_matches_no_flags() {
    let pattern = "may";

    let flags = vec![];

    let files = vec![
        "test_multiple_files_several_matches_no_flags_iliad.txt",
        "test_multiple_files_several_matches_no_flags_midsummer_night.txt",
        "test_multiple_files_several_matches_no_flags_paradise_lost.txt",
    ];

    let expected = vec![
        "test_multiple_files_several_matches_no_flags_midsummer_night.txt:Nor how it may concern my modesty,",
        "test_multiple_files_several_matches_no_flags_midsummer_night.txt:But I beseech your grace that I may know",
        "test_multiple_files_several_matches_no_flags_midsummer_night.txt:The worst that may befall me in this case,",
    ];

    process_grep_case(&pattern, &flags, &files, &expected);
}

#[test]
#[ignore]
/// Multiple files, several matches, print line numbers flag
fn test_multiple_files_several_matches_print_line_numbers_flag() {
    let pattern = "that";

    let flags = vec!["-n"];

    let files = vec![
        "test_multiple_files_several_matches_print_line_numbers_flag_iliad.txt",
        "test_multiple_files_several_matches_print_line_numbers_flag_midsummer_night.txt",
        "test_multiple_files_several_matches_print_line_numbers_flag_paradise_lost.txt",
    ];

    let expected = vec![
        "test_multiple_files_several_matches_print_line_numbers_flag_midsummer_night.txt:5:But I beseech your grace that I may know",
        "test_multiple_files_several_matches_print_line_numbers_flag_midsummer_night.txt:6:The worst that may befall me in this case,",
        "test_multiple_files_several_matches_print_line_numbers_flag_paradise_lost.txt:2:Of that Forbidden Tree, whose mortal tast",
        "test_multiple_files_several_matches_print_line_numbers_flag_paradise_lost.txt:6:Sing Heav'nly Muse, that on the secret top",
    ];

    process_grep_case(&pattern, &flags, &files, &expected);
}

#[test]
#[ignore]
/// Multiple files, one match, print file names flag
fn test_multiple_files_one_match_print_file_names_flag() {
    let pattern = "who";

    let flags = vec!["-l"];

    let files = vec![
        "test_multiple_files_one_match_print_file_names_flag_iliad.txt",
        "test_multiple_files_one_match_print_file_names_flag_midsummer_night.txt",
        "test_multiple_files_one_match_print_file_names_flag_paradise_lost.txt",
    ];

    let expected = vec![
        "test_multiple_files_one_match_print_file_names_flag_iliad.txt",
        "test_multiple_files_one_match_print_file_names_flag_paradise_lost.txt",
    ];

    process_grep_case(&pattern, &flags, &files, &expected);
}

#[test]
#[ignore]
/// Multiple files, several matches, case-insensitive flag
fn test_multiple_files_several_matches_caseinsensitive_flag() {
    let pattern = "TO";

    let flags = vec!["-i"];

    let files = vec![
        "test_multiple_files_several_matches_caseinsensitive_flag_iliad.txt",
        "test_multiple_files_several_matches_caseinsensitive_flag_midsummer_night.txt",
        "test_multiple_files_several_matches_caseinsensitive_flag_paradise_lost.txt",
    ];

    let expected = vec![
        "test_multiple_files_several_matches_caseinsensitive_flag_iliad.txt:Caused to Achaia's host, sent many a soul",
        "test_multiple_files_several_matches_caseinsensitive_flag_iliad.txt:Illustrious into Ades premature,",
        "test_multiple_files_several_matches_caseinsensitive_flag_iliad.txt:And Heroes gave (so stood the will of Jove)",
        "test_multiple_files_several_matches_caseinsensitive_flag_iliad.txt:To dogs and to all ravening fowls a prey,",
        "test_multiple_files_several_matches_caseinsensitive_flag_midsummer_night.txt:I do entreat your grace to pardon me.",
        "test_multiple_files_several_matches_caseinsensitive_flag_midsummer_night.txt:In such a presence here to plead my thoughts;",
        "test_multiple_files_several_matches_caseinsensitive_flag_midsummer_night.txt:If I refuse to wed Demetrius.",
        "test_multiple_files_several_matches_caseinsensitive_flag_paradise_lost.txt:Brought Death into the World, and all our woe,",
        "test_multiple_files_several_matches_caseinsensitive_flag_paradise_lost.txt:Restore us, and regain the blissful Seat,",
        "test_multiple_files_several_matches_caseinsensitive_flag_paradise_lost.txt:Sing Heav'nly Muse, that on the secret top",
    ];

    process_grep_case(&pattern, &flags, &files, &expected);
}

#[test]
#[ignore]
fn test_multiple_files_several_matches_caseinsensitive_flag_utf8() {
    let pattern = "В"; // This letter stands for cyrillic 'Ve' and not latin 'B'. Therefore there should be no matches from paradise_lost.txt

    let flags = vec!["-i"];

    let files = vec![
        "test_multiple_files_several_matches_caseinsensitive_flag_utf8_in_the_white_night.txt",
        "test_multiple_files_several_matches_caseinsensitive_flag_utf8_paradise_lost.txt",
    ];

    let expected = vec![
        "test_multiple_files_several_matches_caseinsensitive_flag_utf8_in_the_white_night.txt:Выплывает в синеве.",
        "test_multiple_files_several_matches_caseinsensitive_flag_utf8_in_the_white_night.txt:Отражается в Неве.",
        "test_multiple_files_several_matches_caseinsensitive_flag_utf8_in_the_white_night.txt:Мне провидится и снится",
        "test_multiple_files_several_matches_caseinsensitive_flag_utf8_in_the_white_night.txt:В вас ли доброе таится,",
    ];

    process_grep_case(&pattern, &flags, &files, &expected);
}

#[test]
#[ignore]
/// Multiple files, several matches, inverted flag
fn test_multiple_files_several_matches_inverted_flag() {
    let pattern = "a";

    let flags = vec!["-v"];

    let files = vec![
        "test_multiple_files_several_matches_inverted_flag_iliad.txt",
        "test_multiple_files_several_matches_inverted_flag_midsummer_night.txt",
        "test_multiple_files_several_matches_inverted_flag_paradise_lost.txt",
    ];

    let expected = vec![
        "test_multiple_files_several_matches_inverted_flag_iliad.txt:Achilles sing, O Goddess! Peleus' son;",
        "test_multiple_files_several_matches_inverted_flag_iliad.txt:The noble Chief Achilles from the son",
        "test_multiple_files_several_matches_inverted_flag_midsummer_night.txt:If I refuse to wed Demetrius.",
    ];

    process_grep_case(&pattern, &flags, &files, &expected);
}

#[test]
#[ignore]
/// Multiple files, one match, match entire lines flag
fn test_multiple_files_one_match_match_entire_lines_flag() {
    let pattern = "But I beseech your grace that I may know";

    let flags = vec!["-x"];

    let files = vec![
        "test_multiple_files_one_match_match_entire_lines_flag_iliad.txt",
        "test_multiple_files_one_match_match_entire_lines_flag_midsummer_night.txt",
        "test_multiple_files_one_match_match_entire_lines_flag_paradise_lost.txt",
    ];

    let expected = vec!["test_multiple_files_one_match_match_entire_lines_flag_midsummer_night.txt:But I beseech your grace that I may know"];

    process_grep_case(&pattern, &flags, &files, &expected);
}

#[test]
#[ignore]
/// Multiple files, one match, multiple flags
fn test_multiple_files_one_match_multiple_flags() {
    let pattern = "WITH LOSS OF EDEN, TILL ONE GREATER MAN";

    let flags = vec!["-n", "-i", "-x"];

    let files = vec![
        "test_multiple_files_one_match_multiple_flags_iliad.txt",
        "test_multiple_files_one_match_multiple_flags_midsummer_night.txt",
        "test_multiple_files_one_match_multiple_flags_paradise_lost.txt",
    ];

    let expected = vec!["test_multiple_files_one_match_multiple_flags_paradise_lost.txt:4:With loss of Eden, till one greater Man"];

    process_grep_case(&pattern, &flags, &files, &expected);
}

#[test]
#[ignore]
/// Multiple files, no matches, various flags
fn test_multiple_files_no_matches_various_flags() {
    let pattern = "Frodo";

    let flags = vec!["-n", "-i", "-x", "-l"];

    let files = vec![
        "test_multiple_files_no_matches_various_flags_iliad.txt",
        "test_multiple_files_no_matches_various_flags_midsummer_night.txt",
        "test_multiple_files_no_matches_various_flags_paradise_lost.txt",
    ];

    let expected = vec![];

    process_grep_case(&pattern, &flags, &files, &expected);
}
