use anyhow::Result;
use hyper::body::Body;
use serde::Serialize;
use serde_json::json;
use std::collections::HashMap;
use std::default::Default;

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
pub struct Container<T> {
    pub builder: T,
}

#[derive(Debug, Serialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CreateContainerBody {
    pub image: String,
    pub cmd: Vec<String>,
}

impl<T: RequestBuilder> Container<T> {
    pub fn new(builder: T) -> Self {
        Container { builder }
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

    pub async fn remove_container(&self, id: &str) -> Result<String> {
        self.builder
            .delete(&format!("/containers/{}", id), None)
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
