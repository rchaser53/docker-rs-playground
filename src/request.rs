use anyhow::{anyhow, Result};
use async_trait::async_trait;
use base64::encode;
use futures_util::stream::TryStreamExt;

use hyper::body::Body;
use hyper::Request;
use hyperlocal::Uri;

use serde::Serialize;
use std::collections::HashMap;

use crate::builder::Builder;

#[async_trait]
impl RequestBuilder for Builder {
    async fn get(&self, target_url: &str) -> Result<String> {
        let url: Uri = Uri::new(&self.base_url, target_url).into();
        let request = Request::builder()
            .method("GET")
            .uri(url)
            .body(Body::empty())
            .unwrap();
        let response_body = self.client.request(request).await?.into_body();
        let bytes = response_body
            .try_fold(Vec::default(), |mut v, bytes| async {
                v.extend(bytes);
                Ok(v)
            })
            .await
            .unwrap();
        Ok(String::from_utf8(bytes).unwrap())
    }

    async fn post<S>(
        &self,
        target_url: &str,
        body: S,
        headers: Option<HashMap<String, String>>,
    ) -> Result<String>
    where
        S: Into<Body> + Send,
    {
        let url: Uri = Uri::new(&self.base_url, target_url).into();
        let mut request = Request::builder().method("POST").uri(url);

        if let Some(headers) = headers {
            for (key, val) in headers.iter() {
                request = request.header(key, val);
            }
        }

        let request = request.body(body.into())?;
        let response_body = self.client.request(request).await?.into_body();
        let bytes = response_body
            .try_fold(Vec::default(), |mut v, bytes| async {
                v.extend(bytes);
                Ok(v)
            })
            .await
            .unwrap();
        Ok(String::from_utf8(bytes).unwrap())
    }
}

#[async_trait]
pub trait RequestBuilder {
    async fn get(&self, target_url: &str) -> Result<String>;
    async fn post<S>(
        &self,
        target_url: &str,
        body: S,
        headers: Option<HashMap<String, String>>,
    ) -> Result<String>
    where
        S: Into<Body> + Send;
}

pub fn serialize_base64<T>(input: T) -> Result<String>
where
    T: Serialize,
{
    let json = serde_json::to_string(&input).map_err(|err| anyhow!(err))?;
    Ok(encode(&json))
}

#[derive(Serialize)]
pub struct XRegistryAuth {
    pub username: String,
    pub password: String,
    pub email: String,
}

impl XRegistryAuth {
    pub fn new(username: String, password: String, email: String) -> Self {
        XRegistryAuth {
            username,
            password,
            email,
        }
    }
}

#[test]
fn serialize_base64_test() {
    let input = XRegistryAuth::new(
        String::from("test_name"),
        String::from("test_password"),
        String::from("test@gmail.com"),
    );
    let expected = encode(
        "{\"username\":\"test_name\",\"password\":\"test_password\",\"email\":\"test@gmail.com\"}",
    );
    assert_eq!(serialize_base64(input).unwrap(), expected);
}
