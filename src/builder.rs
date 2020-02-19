use anyhow::{bail, Result};
use futures_util::stream::TryStreamExt;

use hyper::body::Body;
use hyper::{Client, Request};
use hyperlocal::{UnixClientExt, UnixConnector, Uri};

use crate::images::Image;

#[derive(Debug)]
pub struct Builder {
    pub base_url: String,
    pub client: Client<UnixConnector>,
}

impl Builder {
    pub fn new(base_url: String) -> Self {
        let client = Client::unix();
        Builder { base_url, client }
    }

    pub async fn get(&self, target_url: &str) -> Result<String> {
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

    pub async fn post<S>(&self, target_url: &str, body: S) -> Result<String>
    where
        S: Into<Body>,
    {
        let url: Uri = Uri::new(&self.base_url, target_url).into();
        let request = Request::builder()
            .method("POST")
            .uri(url)
            .body(body.into())
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

    pub fn image(&self) -> Image {
        Image::new()
    }
}

impl Default for Builder {
    fn default() -> Self {
        let client = Client::unix();
        Builder {
            base_url: String::from("/var/run/docker.sock"),
            client,
        }
    }
}
