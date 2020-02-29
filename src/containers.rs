use anyhow::Result;
use serde::Serialize;
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

#[macro_export]
macro_rules! create_macro {
    ( $macro_name:ident, $name:ident) => {
      #[macro_export]
      macro_rules! $macro_name {
        // ( $( ($field:ident, $val:expr) ),* ) => {
        //     serde_qs::to_string(&$name {
        //         $($field: $val),*,
        //         ..Default::default()
        //     });
        // };
        ( $field:ident, $val:expr ) => {
          serde_qs::to_string(&$name {
              $field: $val,
              ..Default::default()
          });
        };
      }
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

    pub async fn get_containers(&self, query_string: &str) -> Result<String> {
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
