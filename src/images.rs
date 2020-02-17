use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Images {
    pub repo_tags: Vec<String>,
    pub id: String,
}
