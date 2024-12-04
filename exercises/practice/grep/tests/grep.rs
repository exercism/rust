use grep::*;

#[test]
fn nonexistent_file_returns_error() {
    let pattern = "Agamemnon";
    let flags = Flags::new(&[]);
    let files = ["0-1-nonexistent-file-returns-error-iliad.txt"];
    assert!(grep(pattern, &flags, &files).is_err());
}

#[test]
#[ignore]
fn grep_returns_result() {
    let pattern = "Agamemnon";
    let flags = Flags::new(&[]);
    let files = Files::new(&["0-2-grep-returns-result-iliad.txt"]);
    assert!(grep(pattern, &flags, files.as_ref()).is_ok());
}

#[test]
#[ignore]
fn one_file_one_match_no_flags() {
    let pattern = "Agamemnon";
    let flags = Flags::new(&[]);
    let files = Files::new(&["1-1-iliad.txt"]);
    let actual = grep(pattern, &flags, files.as_ref()).unwrap();
    let expected: &[&str] = &["Of Atreus, Agamemnon, King of men."];
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
fn one_file_one_match_print_line_numbers_flag() {
    let pattern = "Forbidden";
    let flags = Flags::new(&["-n"]);
    let files = Files::new(&["1-2-paradise-lost.txt"]);
    let actual = grep(pattern, &flags, files.as_ref()).unwrap();
    let expected: &[&str] = &["2:Of that Forbidden Tree, whose mortal tast"];
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
fn one_file_one_match_case_insensitive_flag() {
    let pattern = "FORBIDDEN";
    let flags = Flags::new(&["-i"]);
    let files = Files::new(&["1-3-paradise-lost.txt"]);
    let actual = grep(pattern, &flags, files.as_ref()).unwrap();
    let expected: &[&str] = &["Of that Forbidden Tree, whose mortal tast"];
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
fn one_file_one_match_print_file_names_flag() {
    let pattern = "Forbidden";
    let flags = Flags::new(&["-l"]);
    let files = Files::new(&["1-4-paradise-lost.txt"]);
    let actual = grep(pattern, &flags, files.as_ref()).unwrap();
    let expected: &[&str] = &["1-4-paradise-lost.txt"];
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
fn one_file_one_match_match_entire_lines_flag() {
    let pattern = "With loss of Eden, till one greater Man";
    let flags = Flags::new(&["-x"]);
    let files = Files::new(&["1-5-paradise-lost.txt"]);
    let actual = grep(pattern, &flags, files.as_ref()).unwrap();
    let expected: &[&str] = &["With loss of Eden, till one greater Man"];
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
fn one_file_one_match_multiple_flags() {
    let pattern = "OF ATREUS, Agamemnon, KIng of MEN.";
    let flags = Flags::new(&["-n", "-i", "-x"]);
    let files = Files::new(&["1-6-iliad.txt"]);
    let actual = grep(pattern, &flags, files.as_ref()).unwrap();
    let expected: &[&str] = &["9:Of Atreus, Agamemnon, King of men."];
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
fn one_file_several_matches_no_flags() {
    let pattern = "may";
    let flags = Flags::new(&[]);
    let files = Files::new(&["1-7-midsummer-night.txt"]);
    let actual = grep(pattern, &flags, files.as_ref()).unwrap();
    let expected: &[&str] = &[
        "Nor how it may concern my modesty,",
        "But I beseech your grace that I may know",
        "The worst that may befall me in this case,",
    ];
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
fn one_file_several_matches_print_line_numbers_flag() {
    let pattern = "may";
    let flags = Flags::new(&["-n"]);
    let files = Files::new(&["1-8-midsummer-night.txt"]);
    let actual = grep(pattern, &flags, files.as_ref()).unwrap();
    let expected: &[&str] = &[
        "3:Nor how it may concern my modesty,",
        "5:But I beseech your grace that I may know",
        "6:The worst that may befall me in this case,",
    ];
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
fn one_file_several_matches_match_entire_lines_flag() {
    let pattern = "may";
    let flags = Flags::new(&["-x"]);
    let files = Files::new(&["1-9-midsummer-night.txt"]);
    let actual = grep(pattern, &flags, files.as_ref()).unwrap();
    let expected: &[&str] = &[];
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
fn one_file_several_matches_case_insensitive_flag() {
    let pattern = "ACHILLES";
    let flags = Flags::new(&["-i"]);
    let files = Files::new(&["1-10-iliad.txt"]);
    let actual = grep(pattern, &flags, files.as_ref()).unwrap();
    let expected: &[&str] = &[
        "Achilles sing, O Goddess! Peleus' son;",
        "The noble Chief Achilles from the son",
    ];
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
fn one_file_several_matches_inverted_flag() {
    let pattern = "Of";
    let flags = Flags::new(&["-v"]);
    let files = Files::new(&["1-11-paradise-lost.txt"]);
    let actual = grep(pattern, &flags, files.as_ref()).unwrap();
    let expected: &[&str] = &[
        "Brought Death into the World, and all our woe,",
        "With loss of Eden, till one greater Man",
        "Restore us, and regain the blissful Seat,",
        "Sing Heav'nly Muse, that on the secret top",
        "That Shepherd, who first taught the chosen Seed",
    ];
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
fn one_file_no_matches_various_flags() {
    let pattern = "Gandalf";
    let flags = Flags::new(&["-n", "-l", "-x", "-i"]);
    let files = Files::new(&["1-12-iliad.txt"]);
    let actual = grep(pattern, &flags, files.as_ref()).unwrap();
    let expected: &[&str] = &[];
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
fn one_file_one_match_file_flag_takes_precedence_over_line_flag() {
    let pattern = "ten";
    let flags = Flags::new(&["-n", "-l"]);
    let files = Files::new(&["1-13-iliad.txt"]);
    let actual = grep(pattern, &flags, files.as_ref()).unwrap();
    let expected: &[&str] = &["1-13-iliad.txt"];
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
fn one_file_several_matches_inverted_and_match_entire_lines_flags() {
    let pattern = "Illustrious into Ades premature,";
    let flags = Flags::new(&["-x", "-v"]);
    let files = Files::new(&["1-14-iliad.txt"]);
    let actual = grep(pattern, &flags, files.as_ref()).unwrap();
    let expected: &[&str] = &[
        "Achilles sing, O Goddess! Peleus' son;",
        "His wrath pernicious, who ten thousand woes",
        "Caused to Achaia's host, sent many a soul",
        "And Heroes gave (so stood the will of Jove)",
        "To dogs and to all ravening fowls a prey,",
        "When fierce dispute had separated once",
        "The noble Chief Achilles from the son",
        "Of Atreus, Agamemnon, King of men.",
    ];
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
fn multiple_files_one_match_no_flags() {
    let pattern = "Agamemnon";
    let flags = Flags::new(&[]);
    let files = Files::new(&[
        "2-1-iliad.txt",
        "2-1-midsummer-night.txt",
        "2-1-paradise-lost.txt",
    ]);
    let actual = grep(pattern, &flags, files.as_ref()).unwrap();
    let expected: &[&str] = &["2-1-iliad.txt:Of Atreus, Agamemnon, King of men."];
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
fn multiple_files_several_matches_no_flags() {
    let pattern = "may";
    let flags = Flags::new(&[]);
    let files = Files::new(&[
        "2-2-iliad.txt",
        "2-2-midsummer-night.txt",
        "2-2-paradise-lost.txt",
    ]);
    let actual = grep(pattern, &flags, files.as_ref()).unwrap();
    let expected: &[&str] = &[
        "2-2-midsummer-night.txt:Nor how it may concern my modesty,",
        "2-2-midsummer-night.txt:But I beseech your grace that I may know",
        "2-2-midsummer-night.txt:The worst that may befall me in this case,",
    ];
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
fn multiple_files_several_matches_print_line_numbers_flag() {
    let pattern = "that";
    let flags = Flags::new(&["-n"]);
    let files = Files::new(&[
        "2-3-iliad.txt",
        "2-3-midsummer-night.txt",
        "2-3-paradise-lost.txt",
    ]);
    let actual = grep(pattern, &flags, files.as_ref()).unwrap();
    let expected: &[&str] = &[
        "2-3-midsummer-night.txt:5:But I beseech your grace that I may know",
        "2-3-midsummer-night.txt:6:The worst that may befall me in this case,",
        "2-3-paradise-lost.txt:2:Of that Forbidden Tree, whose mortal tast",
        "2-3-paradise-lost.txt:6:Sing Heav'nly Muse, that on the secret top",
    ];
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
fn multiple_files_one_match_print_file_names_flag() {
    let pattern = "who";
    let flags = Flags::new(&["-l"]);
    let files = Files::new(&[
        "2-4-iliad.txt",
        "2-4-midsummer-night.txt",
        "2-4-paradise-lost.txt",
    ]);
    let actual = grep(pattern, &flags, files.as_ref()).unwrap();
    let expected: &[&str] = &["2-4-iliad.txt", "2-4-paradise-lost.txt"];
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
fn multiple_files_several_matches_case_insensitive_flag() {
    let pattern = "TO";
    let flags = Flags::new(&["-i"]);
    let files = Files::new(&[
        "2-5-iliad.txt",
        "2-5-midsummer-night.txt",
        "2-5-paradise-lost.txt",
    ]);
    let actual = grep(pattern, &flags, files.as_ref()).unwrap();
    let expected: &[&str] = &[
        "2-5-iliad.txt:Caused to Achaia's host, sent many a soul",
        "2-5-iliad.txt:Illustrious into Ades premature,",
        "2-5-iliad.txt:And Heroes gave (so stood the will of Jove)",
        "2-5-iliad.txt:To dogs and to all ravening fowls a prey,",
        "2-5-midsummer-night.txt:I do entreat your grace to pardon me.",
        "2-5-midsummer-night.txt:In such a presence here to plead my thoughts;",
        "2-5-midsummer-night.txt:If I refuse to wed Demetrius.",
        "2-5-paradise-lost.txt:Brought Death into the World, and all our woe,",
        "2-5-paradise-lost.txt:Restore us, and regain the blissful Seat,",
        "2-5-paradise-lost.txt:Sing Heav'nly Muse, that on the secret top",
    ];
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
fn multiple_files_several_matches_inverted_flag() {
    let pattern = "a";
    let flags = Flags::new(&["-v"]);
    let files = Files::new(&[
        "2-6-iliad.txt",
        "2-6-midsummer-night.txt",
        "2-6-paradise-lost.txt",
    ]);
    let actual = grep(pattern, &flags, files.as_ref()).unwrap();
    let expected: &[&str] = &[
        "2-6-iliad.txt:Achilles sing, O Goddess! Peleus' son;",
        "2-6-iliad.txt:The noble Chief Achilles from the son",
        "2-6-midsummer-night.txt:If I refuse to wed Demetrius.",
    ];
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
fn multiple_files_one_match_match_entire_lines_flag() {
    let pattern = "But I beseech your grace that I may know";
    let flags = Flags::new(&["-x"]);
    let files = Files::new(&[
        "2-7-iliad.txt",
        "2-7-midsummer-night.txt",
        "2-7-paradise-lost.txt",
    ]);
    let actual = grep(pattern, &flags, files.as_ref()).unwrap();
    let expected: &[&str] = &["2-7-midsummer-night.txt:But I beseech your grace that I may know"];
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
fn multiple_files_one_match_multiple_flags() {
    let pattern = "WITH LOSS OF EDEN, TILL ONE GREATER MAN";
    let flags = Flags::new(&["-n", "-i", "-x"]);
    let files = Files::new(&[
        "2-8-iliad.txt",
        "2-8-midsummer-night.txt",
        "2-8-paradise-lost.txt",
    ]);
    let actual = grep(pattern, &flags, files.as_ref()).unwrap();
    let expected: &[&str] = &["2-8-paradise-lost.txt:4:With loss of Eden, till one greater Man"];
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
fn multiple_files_no_matches_various_flags() {
    let pattern = "Frodo";
    let flags = Flags::new(&["-n", "-l", "-x", "-i"]);
    let files = Files::new(&[
        "2-9-iliad.txt",
        "2-9-midsummer-night.txt",
        "2-9-paradise-lost.txt",
    ]);
    let actual = grep(pattern, &flags, files.as_ref()).unwrap();
    let expected: &[&str] = &[];
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
fn multiple_files_several_matches_file_flag_takes_precedence_over_line_number_flag() {
    let pattern = "who";
    let flags = Flags::new(&["-n", "-l"]);
    let files = Files::new(&[
        "2-10-iliad.txt",
        "2-10-midsummer-night.txt",
        "2-10-paradise-lost.txt",
    ]);
    let actual = grep(pattern, &flags, files.as_ref()).unwrap();
    let expected: &[&str] = &["2-10-iliad.txt", "2-10-paradise-lost.txt"];
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
fn multiple_files_several_matches_inverted_and_match_entire_lines_flags() {
    let pattern = "Illustrious into Ades premature,";
    let flags = Flags::new(&["-x", "-v"]);
    let files = Files::new(&[
        "2-11-iliad.txt",
        "2-11-midsummer-night.txt",
        "2-11-paradise-lost.txt",
    ]);
    let actual = grep(pattern, &flags, files.as_ref()).unwrap();
    let expected: &[&str] = &[
        "2-11-iliad.txt:Achilles sing, O Goddess! Peleus' son;",
        "2-11-iliad.txt:His wrath pernicious, who ten thousand woes",
        "2-11-iliad.txt:Caused to Achaia's host, sent many a soul",
        "2-11-iliad.txt:And Heroes gave (so stood the will of Jove)",
        "2-11-iliad.txt:To dogs and to all ravening fowls a prey,",
        "2-11-iliad.txt:When fierce dispute had separated once",
        "2-11-iliad.txt:The noble Chief Achilles from the son",
        "2-11-iliad.txt:Of Atreus, Agamemnon, King of men.",
        "2-11-midsummer-night.txt:I do entreat your grace to pardon me.",
        "2-11-midsummer-night.txt:I know not by what power I am made bold,",
        "2-11-midsummer-night.txt:Nor how it may concern my modesty,",
        "2-11-midsummer-night.txt:In such a presence here to plead my thoughts;",
        "2-11-midsummer-night.txt:But I beseech your grace that I may know",
        "2-11-midsummer-night.txt:The worst that may befall me in this case,",
        "2-11-midsummer-night.txt:If I refuse to wed Demetrius.",
        "2-11-paradise-lost.txt:Of Mans First Disobedience, and the Fruit",
        "2-11-paradise-lost.txt:Of that Forbidden Tree, whose mortal tast",
        "2-11-paradise-lost.txt:Brought Death into the World, and all our woe,",
        "2-11-paradise-lost.txt:With loss of Eden, till one greater Man",
        "2-11-paradise-lost.txt:Restore us, and regain the blissful Seat,",
        "2-11-paradise-lost.txt:Sing Heav'nly Muse, that on the secret top",
        "2-11-paradise-lost.txt:Of Oreb, or of Sinai, didst inspire",
        "2-11-paradise-lost.txt:That Shepherd, who first taught the chosen Seed",
    ];
    assert_eq!(actual, expected);
}

static ILIAD_CONTENT: &str = "\
Achilles sing, O Goddess! Peleus' son;
His wrath pernicious, who ten thousand woes
Caused to Achaia's host, sent many a soul
Illustrious into Ades premature,
And Heroes gave (so stood the will of Jove)
To dogs and to all ravening fowls a prey,
When fierce dispute had separated once
The noble Chief Achilles from the son
Of Atreus, Agamemnon, King of men.
";

static MIDSUMMER_NIGHT_CONTENT: &str = "\
I do entreat your grace to pardon me.
I know not by what power I am made bold,
Nor how it may concern my modesty,
In such a presence here to plead my thoughts;
But I beseech your grace that I may know
The worst that may befall me in this case,
If I refuse to wed Demetrius.
";

static PARADISE_LOST_CONTENT: &str = "\
Of Mans First Disobedience, and the Fruit
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
static IN_THE_WHITE_NIGHT_CONTENT: &str = "
Белой ночью месяц красный
Выплывает в синеве.
Бродит призрачно-прекрасный,
Отражается в Неве.
Мне провидится и снится
Исполненье тайных дум.
В вас ли доброе таится,
Красный месяц, тихий шум?..
";

struct Files<'a> {
    file_names: &'a [&'a str],
}

impl<'a> Files<'a> {
    fn new(file_names: &'a [&'a str]) -> Self {
        for file_name in file_names {
            let content = if file_name.ends_with("iliad.txt") {
                ILIAD_CONTENT
            } else if file_name.ends_with("midsummer-night.txt") {
                MIDSUMMER_NIGHT_CONTENT
            } else if file_name.ends_with("paradise-lost.txt") {
                PARADISE_LOST_CONTENT
            } else {
                IN_THE_WHITE_NIGHT_CONTENT
            };
            std::fs::write(file_name, content).unwrap_or_else(|_| {
                panic!("Error setting up file '{file_name}' with the following content:\n{content}")
            });
        }

        Self { file_names }
    }
}

impl Drop for Files<'_> {
    fn drop(&mut self) {
        for file_name in self.file_names {
            std::fs::remove_file(file_name)
                .unwrap_or_else(|e| panic!("Could not delete file '{file_name}': {e}"));
        }
    }
}

impl<'a> AsRef<[&'a str]> for Files<'a> {
    fn as_ref(&self) -> &[&'a str] {
        self.file_names
    }
}
