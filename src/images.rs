use anyhow::{bail, Result};
use async_trait::async_trait;

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

pub trait Req {}
impl Req for Image {}

#[async_trait]
pub trait IsReq {
    async fn get(&self) -> Result<String>;
}

#[async_trait]
impl<T: Req + Sync> IsReq for T {
    async fn get(&self) -> Result<String> {
        let builder: Builder = Default::default();
        let result = builder.get("/images/json?digests=1").await?;
        Ok(result)
    }
}

pub trait HaveReq {
    type A: IsReq;
    fn get_req(&self) -> &Self::A;
}
impl HaveReq for Image {
    type A = Self;
    fn get_req(&self) -> &Self::A {
        &self
    }
}

mod test {
    use anyhow::{bail, Result};
    use async_trait::async_trait;
    use tokio::runtime::Runtime;

    use super::{Builder, HaveReq, Image, Req};

    #[test]
    fn test_req() {
        #[async_trait]
        pub trait IsReq {
            async fn get(&self) -> Result<String>;
        }

        #[async_trait]
        impl<T: Req + Sync> IsReq for T {
            async fn get(&self) -> Result<String> {
                Ok("test".to_string())
            }
        }
        let mut rt = Runtime::new().unwrap();

        let image = Image::new();
        let result = rt.block_on(image.get_req().get());

        assert_eq!(result.unwrap(), "test");
    }
}
