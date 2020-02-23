use anyhow::{bail, Result};

use crate::builder::Builder;
use crate::request::RequestBuilder;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DockerImage {
    pub repo_tags: Vec<String>,
    pub id: String,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum DockerImagePull {
    Success { status: String },
    Failed { message: String },
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

    pub async fn get_images(&self) -> Result<Vec<DockerImage>> {
        let bytes = self.builder.get("/images/json").await?;
        deserialize_docker_images(&bytes)
    }

    pub async fn pull_image(&self, image_name: &str, tag: &str) -> Result<Vec<DockerImagePull>> {
        let bytes = self
            .builder
            .post(
                &format!("/images/create?fromImage={}&tag={}", image_name, tag),
                vec![],
            )
            .await?;
        deserialize_docker_pull_images(&bytes)
    }
}

pub fn deserialize_docker_images(input: &str) -> Result<Vec<DockerImage>> {
    match serde_json::from_str(&input) {
        Ok(data) => Ok(data),
        Err(err) => bail!("{}", err),
    }
}

pub fn deserialize_docker_pull_images(input: &str) -> Result<Vec<DockerImagePull>> {
    let result = input
        .split("\n")
        .filter(|s| *s != "")
        .map(|s| {
            dbg!(&s);
            let result: DockerImagePull = match serde_json::from_str(&s) {
                Ok(data) => data,
                Err(err) => panic!("{}", err),
            };
            result
        })
        .collect::<Vec<DockerImagePull>>();
    Ok(result)
}

mod test {
    use super::*;

    #[test]
    fn deseriazlie_docker_images_success() {
        let converted = deserialize_docker_images(r#"[{ "RepoTags": [], "Id": "test" }]"#);

        assert_eq!(
            converted.unwrap(),
            vec![DockerImage {
                repo_tags: vec![],
                id: "test".to_string()
            }]
        );
    }

    #[test]
    fn deserialize_docker_pull_images_success() {
        let converted = deserialize_docker_pull_images(
            r#"{ "message": "test" }
{ "status": "succeeded" }"#,
        );

        assert_eq!(
            converted.unwrap(),
            vec![
                DockerImagePull::Failed {
                    message: "test".to_string()
                },
                DockerImagePull::Success {
                    status: "succeeded".to_string()
                }
            ]
        );
    }
}
