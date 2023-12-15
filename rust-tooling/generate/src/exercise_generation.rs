use std::{
    collections::HashMap,
    io::Write,
    process::{Command, Stdio},
};

use tera::{Context, Tera};

use models::{
    exercise_config::get_excluded_tests,
    problem_spec::{get_additional_test_cases, get_canonical_data, SingleTestCase, TestCase},
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

static TEST_TEMPLATE: &str = include_str!("../templates/default_test_template.tera");

fn with_group_description(mut cases: Vec<TestCase>, group_description: &str) -> Vec<TestCase> {
    for case in cases.iter_mut() {
        if let TestCase::Single { case } = case {
            case.group_description = Some(group_description.to_string());
        }
        // Deeper nesting of groups is ignored at this point.
        // As such, single test cases will only have the group description of
        // their immediate parent group.
        // It's highly unlikely that more information will be needed in practice.
    }
    cases
}

fn extend_single_cases(single_cases: &mut Vec<SingleTestCase>, cases: Vec<TestCase>) {
    for case in cases {
        match case {
            TestCase::Single { case } => single_cases.push(case),
            TestCase::Group {
                cases, description, ..
            } => extend_single_cases(single_cases, with_group_description(cases, &description)),
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
    let cases = {
        let mut cases = get_canonical_data(slug)
            .map(|data| data.cases)
            .unwrap_or_default();
        cases.extend_from_slice(&get_additional_test_cases(slug));
        cases
    };
    let excluded_tests = get_excluded_tests(slug);
    let mut template = get_test_template(slug).unwrap();
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

    let rendered = template.render("test_template.tera", &context).unwrap();
    let rendered = rendered.trim_start();

    let mut child = Command::new("rustfmt")
        .args(["--color=always"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to spawn process");

    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(rendered.as_bytes())
        .unwrap();
    let rustfmt_out = child.wait_with_output().unwrap();

    if rustfmt_out.status.success() {
        String::from_utf8(rustfmt_out.stdout).unwrap()
    } else {
        let rustfmt_error = String::from_utf8(rustfmt_out.stderr).unwrap();
        let mut last_16_error_lines = rustfmt_error.lines().rev().take(16).collect::<Vec<_>>();
        last_16_error_lines.reverse();
        let last_16_error_lines = last_16_error_lines.join("\n");

        println!(
            "{last_16_error_lines}\
^ last 16 lines of errors from rustfmt
Check the test template (.meta/test_template.tera)
It probably generates invalid Rust code."
        );

        // still return the unformatted content to be written to the file
        rendered.into()
    }
}

pub fn get_test_template(slug: &str) -> Option<Tera> {
    Some(Tera::new(format!("exercises/practice/{slug}/.meta/*.tera").as_str()).unwrap())
}
