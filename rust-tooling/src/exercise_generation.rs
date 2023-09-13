use std::collections::HashMap;

use tera::Context;

use crate::{
    exercise_config::{get_excluded_tests, get_test_emplate},
    problem_spec::{get_canonical_data, SingleTestCase, TestCase},
};

pub struct GeneratedExercise {
    pub gitignore: String,
    pub manifest: String,
    pub lib_rs: String,
    pub example: String,
    pub test_template: String,
    pub tests: String,
}

pub fn new(slug: &str, fn_names: Vec<String>) -> GeneratedExercise {
    let crate_name = slug.replace('-', "_");
    let first_fn_name = &fn_names[0];

    GeneratedExercise {
        gitignore: GITIGNORE.into(),
        manifest: generate_manifest(&crate_name),
        lib_rs: generate_lib_rs(&crate_name, first_fn_name),
        example: generate_example_rs(first_fn_name),
        test_template: TEST_TEMPLATE.into(),
        tests: generate_tests(slug, fn_names),
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

fn generate_lib_rs(crate_name: &str, fn_name: &str) -> String {
    format!(
        concat!(
            "pub fn {fn_name}(input: TODO) -> TODO {{\n",
            "    todo!(\"use {{input}} to implement {crate_name}\")\n",
            "}}\n",
        ),
        fn_name = fn_name,
        crate_name = crate_name,
    )
}

fn generate_example_rs(fn_name: &str) -> String {
    format!(
        concat!(
            "pub fn {fn_name}(input: TODO) -> TODO {{\n",
            "    TODO\n",
            "}}\n",
        ),
        fn_name = fn_name
    )
}

static TEST_TEMPLATE: &str = include_str!("default_test_template.tera");

fn extend_single_cases(single_cases: &mut Vec<SingleTestCase>, cases: Vec<TestCase>) {
    for case in cases {
        match case {
            TestCase::Single { case } => single_cases.push(case),
            TestCase::Group { cases, .. } => extend_single_cases(single_cases, cases),
        }
    }
}

fn to_hex(value: &tera::Value, _args: &HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
    Ok(serde_json::Value::String(format!(
        "{:x}",
        value.as_u64().unwrap()
    )))
}

fn generate_tests(slug: &str, fn_names: Vec<String>) -> String {
    let cases = get_canonical_data(slug).cases;
    let excluded_tests = get_excluded_tests(slug);
    let mut template = get_test_emplate(slug).unwrap();
    if template.get_template_names().next().is_none() {
        template
            .add_raw_template("test_template.tera", TEST_TEMPLATE)
            .unwrap();
    }
    template.register_filter("to_hex", to_hex);

    let mut single_cases = Vec::new();
    extend_single_cases(&mut single_cases, cases);
    single_cases.retain(|case| !excluded_tests.contains(&case.uuid));

    let mut context = Context::new();
    context.insert("crate_name", &slug.replace('-', "_"));
    context.insert("fn_names", &fn_names);
    context.insert("cases", &single_cases);

    template
        .render("test_template.tera", &context)
        .unwrap()
        .trim_start()
        .into()
}
