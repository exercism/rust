use convert_case::{Case, Casing};

use crate::{problem_spec::{get_canonical_data, SingleTestCase, TestCase}, exercise_config::get_excluded_tests};

pub struct GeneratedExercise {
    pub gitignore: String,
    pub manifest: String,
    pub lib_rs: String,
    pub example: String,
    pub test_header: String,
    pub test_cases: String,
}

pub fn new(slug: &str) -> GeneratedExercise {
    let crate_name = slug.replace('-', "_");

    GeneratedExercise {
        gitignore: GITIGNORE.into(),
        manifest: generate_manifest(&crate_name),
        lib_rs: generate_lib_rs(&crate_name),
        example: EXAMPLE_RS.into(),
        test_header: generate_test_header(&crate_name),
        test_cases: generate_tests(slug),
    }
}

static GITIGNORE: &str = "\
/target
/Cargo.lock
";

fn generate_manifest(crate_name: &str) -> String {
    format!(
        concat!(
            "[package]\n",
            "edition = \"2021\"\n",
            "name = \"{crate_name}\"\n",
            "version = \"1.0.0\"\n",
            "\n",
            "[dependencies]\n",
        ),
        crate_name = crate_name
    )
}

fn generate_lib_rs(crate_name: &str) -> String {
    format!(
        concat!(
            "pub fn TODO(input: TODO) -> TODO {{\n",
            "   todo!(\"use {{input}} to implement {crate_name}\")\n",
            "}}\n",
        ),
        crate_name = crate_name
    )
}

static EXAMPLE_RS: &str = "\
pub fn TODO(input: TODO) -> TODO {
    TODO
}
";

fn generate_test_header(crate_name: &str) -> String {
    format!(
        concat!(
            "use {crate_name}::*;\n",
            "\n",
            "fn process(input: TODO, expected: TODO) -> bool {{\n",
            "   TODO\n",
            "}}\n",
            "\n",
            "// The following test cases are generated from problem-specifications.\n",
            "// If you'd like to improve the test suite, you can do it over there.\n",
            "// https://github.com/exercism/problem-specifications/\n",
        ),
        crate_name = crate_name
    )
}

fn extend_single_cases(single_cases: &mut Vec<SingleTestCase>, cases: Vec<TestCase>) {
    for case in cases {
        match case {
            TestCase::Single { case } => single_cases.push(case),
            TestCase::Group { cases, .. } => extend_single_cases(single_cases, cases),
        }
    }
}

fn generate_single_test_case(case: SingleTestCase, is_first: bool) -> String {
    // TODO generate tests with an author-provided template (Tera?)
    let fn_name = case.description.to_case(Case::Snake);
    format!(
        concat!(
            "\n",
            "#[test]\n",
            "{ignore}",
            "fn {fn_name}() {{\n",
            "   process({input}, {expected})\n",
            "}}\n",
        ),
        ignore = if is_first { "" } else { "#[ignore]\n" },
        fn_name = fn_name,
        input = case.input,
        expected = case.expected,
    )
}

fn generate_tests(slug: &str) -> String {
    let cases = get_canonical_data(slug).cases;
    let excluded_tests = get_excluded_tests(slug);

    let mut single_cases = Vec::new();
    extend_single_cases(&mut single_cases, cases);
    single_cases.retain(|case| !excluded_tests.contains(&case.uuid));

    let mut buffer = String::new();

    buffer.push_str(generate_single_test_case(single_cases.remove(0), true).as_str());

    for case in single_cases {
        buffer.push_str(generate_single_test_case(case, false).as_str());
    }

    buffer
}
