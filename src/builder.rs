use futures_util::stream::TryStreamExt;
use hyper::body::Body;
use hyper::Client;
use hyperlocal::{UnixClientExt, UnixConnector, Uri};

use crate::images::Image;

#[derive(Debug)]
pub struct Builder {
    pub base_url: String,
    pub client: Client<UnixConnector, Body>,
}

impl Builder {
    pub fn new(base_url: String) -> Self {
        let client = Client::unix();
        Builder { base_url, client }
    }

    pub async fn get(
        &self,
        target_url: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let url = Uri::new(&self.base_url, target_url).into();
        let response_body = self.client.get(url).await?.into_body();

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
