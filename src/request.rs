use anyhow::Result;
use async_trait::async_trait;
use futures_util::stream::TryStreamExt;

use hyper::body::Body;
use hyper::Request;
use hyperlocal::Uri;

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

    async fn post<S: Into<Body> + Send>(&self, target_url: &str, body: S) -> Result<String> {
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
}

#[async_trait]
pub trait RequestBuilder {
    async fn get(&self, target_url: &str) -> Result<String>;
    async fn post<S: Into<Body> + Send>(&self, target_url: &str, body: S) -> Result<String>;
}
