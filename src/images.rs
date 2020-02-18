use crate::builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DockerImage {
    pub repo_tags: Vec<String>,
    pub id: String,
}

#[derive(Debug)]
pub struct Image {
    pub builder: Builder,
}

impl Image {
    pub fn new() -> Self {
        Image {
            builder: Default::default(),
        }
    }

    pub async fn get_images(&self) -> Vec<DockerImage> {
        let bytes = self.builder.get("/images/json").await.unwrap();

        match serde_json::from_str(&bytes) {
            Ok(data) => data,
            Err(err) => panic!("{}", err),
        }
    }
}
