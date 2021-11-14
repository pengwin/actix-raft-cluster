mod error;

use reqwest::{Client as RequestClient, StatusCode};
use url::Url;

use error::ClientError;

use crate::config::Config;

pub struct Client {
    client: RequestClient,
    base_url: Url,
}

impl Client {
    pub fn new(cfg: &Config) -> Result<Client, ClientError> {
        let client = RequestClient::builder().build()?;
        let base_url = Url::parse(&cfg.client_endpoint)?;
        Ok(Client { client, base_url })
    }

    pub async fn health_check(&self) -> Result<String, ClientError> {
        let url = self.base_url.join("/healthcheck")?;
        let res = self.client.get(url).send().await?;

        let status_code = res.status();
        let message = res.text().await?;

        if status_code != StatusCode::OK {
            return Err(ClientError::HealthCheckError{status_code, message });
        }
        
        Ok(message)
    }
}
