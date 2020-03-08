use anyhow::{bail, Result};

use crate::builder::Builder;
use crate::request::{serialize_base64, RequestBuilder, XRegistryAuth};
use serde::Deserialize;
use std::collections::HashMap;

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

    pub async fn get_images(&self) -> Result<String> {
        self.builder.get("/images/json", None).await
    }

    pub async fn pull_image(
        &self,
        image_name: &str,
        tag: &str,
        auth_info: Option<XRegistryAuth>,
    ) -> Result<String> {
        let auth_info = if let Some(x_registry_auth) = auth_info {
            let mut map = HashMap::new();
            let auth_token_str =
                serialize_base64(x_registry_auth).expect("failed to serialize to base64");
            map.insert(String::from("X-Registry-Auth"), auth_token_str);
            Some(map)
        } else {
            None
        };

        self.builder
            .post(
                &format!("/images/create?fromImage={}&tag={}", image_name, tag),
                vec![],
                auth_info,
            )
            .await
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
