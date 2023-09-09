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

pub fn get_canonical_data(slug: &str) -> CanonicalData {
    crate::fs_utils::cd_into_repo_root();
    let path = std::path::PathBuf::from("problem-specifications/exercises")
        .join(slug)
        .join("canonical-data.json");
    let contents = std::fs::read_to_string(&path).unwrap();
    serde_json::from_str(contents.as_str()).unwrap_or_else(|e| {
        panic!(
            "should deserialize canonical data for {}: {e}",
            path.display()
        )
    })
}

#[test]
fn test_deserialize_canonical_data() {
    crate::fs_utils::cd_into_repo_root();
    for entry in walkdir::WalkDir::new("problem-specifications/exercises")
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
