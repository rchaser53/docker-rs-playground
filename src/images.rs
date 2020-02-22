use anyhow::{bail, Result};

use crate::builder::Builder;
use crate::request::RequestBuilder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DockerImage {
    pub repo_tags: Vec<String>,
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
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
        let bytes = self.builder.get("/images/json?digests=1").await?;

        match serde_json::from_str(&bytes) {
            Ok(data) => Ok(data),
            Err(err) => bail!("{}", err),
        }
    }

    pub async fn pull_image(&self, image_name: &str, tag: &str) -> Result<Vec<DockerImagePull>> {
        let result = self
            .builder
            .post(
                &format!("/images/create?fromImage={}&tag={}", image_name, tag),
                vec![],
            )
            .await?
            .split("\r\n")
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
}

struct Hub {}

pub trait SvcA {}
impl SvcA for Hub {}

pub trait IsSvcA {
    fn a(&self) -> String;
}
impl<T: SvcA> IsSvcA for T {
    fn a(&self) -> String {
        "svc-a".to_string()
    }
}

pub trait HaveSvcA {
    type A: IsSvcA;
    fn get_svc_a(&self) -> &Self::A;
}
impl HaveSvcA for Hub {
    type A = Self;
    fn get_svc_a(&self) -> &Self::A {
        &self
    }
}

mod test {
    use super::{HaveSvcA, Hub, SvcA};

    #[test]
    fn test_use_b() {
        trait IsSvcA {
            fn a(&self) -> String;
        }
        impl<T: SvcA> IsSvcA for T {
            fn a(&self) -> String {
                "svc-d".to_string()
            }
        }

        let svc = Hub {};
        assert_eq!(svc.get_svc_a().a(), "svc-a");
    }
}
