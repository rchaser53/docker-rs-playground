use anyhow::Result;

use crate::builder::Builder;
use crate::request::RequestBuilder;

#[derive(Debug)]
pub struct Container {
    pub builder: Builder,
}

impl Container {
    pub fn new() -> Self {
        Container {
            builder: Default::default(),
        }
    }

    pub async fn get_containers(&self) -> Result<String> {
        self.builder.get("/containers/json").await
    }

    pub async fn get_container(&self, id: &str) -> Result<String> {
        self.builder.get(&format!("/containers/{}/json", id)).await
    }
}
