use anyhow::Result;
use hyper::body::Body;
use serde::Serialize;
use serde_json::json;
use std::collections::HashMap;
use std::default::Default;

use crate::builder::Builder;
use crate::request::RequestBuilder;

#[macro_export]
macro_rules! create_query_by_struct {
  ( $name:ident, $( ($field:ident, $val:expr) ),* ) => {
      serde_qs::to_string(&$name {
          $($field: $val),*,
          ..Default::default()
      });
  };
  ( $name:ident ) => {
    serde_qs::to_string(&$name {
        ..Default::default()
    });
  }
}

#[derive(Debug)]
pub struct Container {
    pub builder: Builder,
}

#[derive(Debug, Serialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CreateContainerBody {
    pub image: String,
    pub cmd: Vec<String>,
}

impl Container {
    pub fn new() -> Self {
        Container {
            builder: Default::default(),
        }
    }

    pub async fn get_containers(&self, query_string: &str) -> Result<String> {
        self.builder
            .get(&format!("/containers/json?{}", query_string), None)
            .await
    }

    pub async fn get_container(&self, id: &str, query_string: &str) -> Result<String> {
        self.builder
            .get(&format!("/containers/{}/json?{}", id, query_string), None)
            .await
    }

    pub async fn create_container(
        &self,
        container_info: CreateContainerBody,
        query_string: &str,
    ) -> Result<String> {
        let body = Body::from(json!(container_info).to_string());
        let mut map = HashMap::new();
        map.insert(
            String::from("Content-Type"),
            String::from("application/json"),
        );

        self.builder
            .post(
                &format!("/containers/create?{}", query_string),
                body,
                Some(map),
            )
            .await
    }
}

#[derive(Debug, Default, Serialize)]
pub struct GetContainerOption {
    pub all: Option<bool>,
    pub limit: Option<usize>,
    pub since: Option<String>,
    pub before: Option<String>,
    pub size: Option<bool>,
    pub filters: Option<String>,
}
