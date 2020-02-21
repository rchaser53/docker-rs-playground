use hyper::Client;
use hyperlocal::{UnixClientExt, UnixConnector};

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
