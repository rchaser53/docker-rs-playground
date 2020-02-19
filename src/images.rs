use crate::builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DockerImage {
    pub repo_tags: Vec<String>,
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DockerImagePull {
    pub status: String,
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
        let bytes = self.builder.get("/images/json?digests=1").await.unwrap();

        match serde_json::from_str(&bytes) {
            Ok(data) => data,
            Err(err) => panic!("{}", err),
        }
    }

    pub async fn pull_image(&self, image_name: &str, tag: &str) -> Vec<String> {
        let bytes = self
            .builder
            .post(
                &format!("/images/create?fromImage={}&tag={}", image_name, tag),
                vec![],
            )
            .await
            .unwrap();
        bytes
            .split("\r\n")
            .filter(|s| *s != "")
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>()
    }
}
