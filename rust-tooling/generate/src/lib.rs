use std::{
    io::Write,
    process::{Command, Stdio},
};

use anyhow::{Context, Result};
use tera::Tera;

use custom_filters::CUSTOM_FILTERS;
use models::{
    exercise_config::get_excluded_tests,
    problem_spec::{get_additional_test_cases, get_canonical_data, TestCase},
};

mod custom_filters;

pub struct GeneratedExercise {
    pub gitignore: String,
    pub manifest: String,
    pub lib_rs: String,
    pub example: String,
    pub test_template: String,
    pub tests: String,
}

pub fn new(slug: &str) -> Result<GeneratedExercise> {
    let crate_name = slug.replace('-', "_");

    Ok(GeneratedExercise {
        gitignore: GITIGNORE.into(),
        manifest: generate_manifest(&crate_name),
        lib_rs: LIB_RS.into(),
        example: EXAMPLE_RS.into(),
        test_template: TEST_TEMPLATE.into(),
        tests: generate_tests(slug)?,
    })
}

static GITIGNORE: &str = "\
/target
Cargo.lock
";

fn generate_manifest(crate_name: &str) -> String {
    format!(
        concat!(
            "[package]\n",
            "edition = \"2021\"\n",
            "name = \"{crate_name}\"\n",
            "version = \"1.0.0\"\n",
        ),
        crate_name = crate_name
    )
}

static LIB_RS: &str = "\
pub fn TODO(input: TODO) -> TODO {
    todo!(\"use {input} to solve the exercise\")
}
";

static EXAMPLE_RS: &str = "\
pub fn TODO(input: TODO) -> TODO {
    TODO
}
";

static TEST_TEMPLATE: &str = include_str!("../templates/default_test_template.tera");

fn remove_excluded_tests(cases: &mut Vec<TestCase>, excluded_tests: &[String]) {
    cases.retain(|case| match case {
        TestCase::Single { case } => !excluded_tests.contains(&case.uuid),
        _ => true,
    });
    for case in cases {
        if let TestCase::Group { cases, .. } = case {
            remove_excluded_tests(cases, excluded_tests)
        }
    }
}

fn generate_tests(slug: &str) -> Result<String> {
    let mut cases = {
        let mut cases = get_canonical_data(slug)
            .map(|data| data.cases)
            .unwrap_or_default();
        cases.extend_from_slice(&get_additional_test_cases(slug));
        cases
    };
    let excluded_tests = get_excluded_tests(slug);
    let mut template = get_test_template(slug).context("failed to get test template")?;
    if template.get_template_names().next().is_none() {
        template
            .add_raw_template("test_template.tera", TEST_TEMPLATE)
            .context("failed to add default template")?;
    }
    for (name, filter) in CUSTOM_FILTERS {
        template.register_filter(name, filter);
    }

    remove_excluded_tests(&mut cases, &excluded_tests);

    let mut context = tera::Context::new();
    context.insert("cases", &cases);

    let rendered = template
        .render("test_template.tera", &context)
        .with_context(|| format!("failed to render template of '{slug}'"))?;

    // Remove ignore-annotation on first test.
    // This could be done in the template itself,
    // but doing it here makes all templates more readable.
    // Also, it is harder to do this in the template when the template
    // generates test functions inside a macro for modules.
    let rendered = rendered.replacen("#[ignore]\n", "", 1);

    let mut child = Command::new("rustfmt")
        .args(["--color=always"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context("failed to spawn rustfmt process")?;

    child
        .stdin
        .as_mut()
        .context("failed to get rustfmt's stdin")?
        .write_all(rendered.as_bytes())
        .context("failed to write to rustfmt's stdin")?;
    let rustfmt_out = child
        .wait_with_output()
        .context("failed to get rustfmt's output")?;

    if !rustfmt_out.status.success() {
        let rustfmt_error = String::from_utf8_lossy(&rustfmt_out.stderr);
        let mut last_16_error_lines = rustfmt_error.lines().rev().take(16).collect::<Vec<_>>();
        last_16_error_lines.reverse();
        let last_16_error_lines = last_16_error_lines.join("\n");

        eprintln!(
            "{last_16_error_lines}\
^ last 16 lines of errors from rustfmt
Check the test template (.meta/test_template.tera)
It probably generates invalid Rust code."
        );

        // still return the unformatted content to be written to the file
        return Ok(rendered);
    }
    Ok(String::from_utf8_lossy(&rustfmt_out.stdout).into_owned())
}

pub fn get_test_template(slug: &str) -> Result<Tera> {
    Tera::new(format!("exercises/practice/{slug}/.meta/*.tera").as_str()).map_err(Into::into)
}
