use anyhow::Result;
use serde::Serialize;
use std::default::Default;

use crate::builder::Builder;
use crate::request::RequestBuilder;

macro_rules! create_query_by_struct {
  ( $name:ident, $( ($field:ident, $val:expr) ),* ) => {
      serde_qs::to_string(&$name {
          $($field: $val),*,
          ..Default::default()
      });
  }
}

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
        let query_string = create_query_by_struct!(GetContainerOption, (all, Some(false))).unwrap();
        self.builder
            .get(&format!("/containers/json?{}", query_string))
            .await
    }

    pub async fn get_container(&self, id: &str) -> Result<String> {
        self.builder.get(&format!("/containers/{}/json", id)).await
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

// all – 1/True/true or 0/False/false, Show all containers. Only running containers are shown by default (i.e., this defaults to false)
// limit – Show limit last created containers, include non-running ones.
// since – Show only containers created since Id, include non-running ones.
// before – Show only containers created before Id, include non-running ones.
// size – 1/True/true or 0/False/false, Show the containers sizes
// filters - a JSON encoded value of the filters (a map[string][]string) to process on the containers list. Available filters:
