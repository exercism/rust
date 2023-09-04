use serde::{Deserialize, Serialize};

/// Remember that this is actually optional, not all exercises
/// must have a canonical data file in the problem-specifications repo.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CanonicalData {
    pub exercise: String,
    pub comments: Option<Vec<String>>,
    pub cases: Vec<TestCase>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum TestCase {
    Single {
        #[serde(flatten)]
        case: SingleTestCase,
    },
    Group {
        description: String,
        comments: Option<Vec<String>>,
        cases: Vec<TestCase>,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SingleTestCase {
    pub uuid: String,
    pub reimplements: Option<String>,
    pub description: String,
    pub comments: Option<Vec<String>>,
    pub scenarios: Option<Vec<String>>,
    pub property: String,
    pub input: serde_json::Value,
    pub expected: serde_json::Value,
}

/// Ignored because the problem-specifications repository may not be present.
#[test]
#[ignore]
fn test_deserialize_all() {
    let spec_dir =
        env!("HOME").to_string() + "/.cache/exercism/configlet/problem-specifications/exercises";
    for entry in walkdir::WalkDir::new(spec_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name().to_str().unwrap() == "canonical-data.json")
    {
        let contents = std::fs::read_to_string(entry.path()).unwrap();
        let _: CanonicalData = serde_json::from_str(contents.as_str()).unwrap_or_else(|e| {
            panic!(
                "should deserialize canonical data for {}: {e}",
                entry.path().display()
            )
        });
    }
}
