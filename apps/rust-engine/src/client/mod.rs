use reqwest::{Client as ReqwestClient, ClientBuilder};
use std::time::Duration;

pub struct Client {
    inner: ReqwestClient,
}

impl Client {
    pub fn new() -> Self {
        let client = ClientBuilder::new()
            .timeout(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(10))
            .build()
            .expect("Failed to create HTTP client");

        Self { inner: client }
    }

    pub fn inner(&self) -> &ReqwestClient {
        &self.inner
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}
