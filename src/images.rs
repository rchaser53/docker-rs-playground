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

    pub async fn pull_image(&self) -> String {
        let bytes = self
            .builder
            .post("/images/create?fromImage=busybox&tag=latest", vec![])
            .await
            .unwrap();

        bytes

        // match serde_json::from_str(&bytes) {
        //     Ok(data) => data,
        //     Err(err) => panic!("{}", err),
        // }
    }
}
