use reqwest::StatusCode;
use std::time::Duration;
use thiserror::Error;

use nitronanite_models::Message;
use nitronanite_snowflake::Snowflake;

const DEFAULT_BASE_URL: &str = "https://discord.com/api/v9";
const DEFAULT_TIMEOUT_SECS: u64 = 30;
const USER_AGENT: &str = "Nitronanite/1.0";

#[derive(Debug, Clone)]
pub struct Http {
    client: reqwest::Client,
    token: String,
    base_url: String,
}

#[derive(Debug, Clone)]
pub struct HttpBuilder {
    token: Option<String>,
    user_agent: Option<String>,
    timeout: Duration,
    base_url: String,
}

#[derive(Debug, Error)]
pub enum ClientBuildError {
    #[error("missing discord token")]
    MissingToken,
    #[error("discord token cannot be empty")]
    EmptyToken,
    #[error("missing user-agent")]
    MissingUserAgent,
    #[error("user-agent cannot be empty")]
    EmptyUserAgent,
    #[error("failed to build reqwest client: {0}")]
    RequestClient(reqwest::Error),
}

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("request failed: {0}")]
    Request(reqwest::Error),
    #[error("discord api returned {status}: {body}")]
    Status { status: StatusCode, body: String },
}

impl Default for HttpBuilder {
    fn default() -> Self {
        Self {
            token: None,
            user_agent: None,
            timeout: Duration::from_secs(DEFAULT_TIMEOUT_SECS),
            base_url: DEFAULT_BASE_URL.to_owned(),
        }
    }
}

impl HttpBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self
    }

    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    pub fn build(self) -> Result<Http, ClientBuildError> {
        let token = match self.token {
            Some(token) if !token.trim().is_empty() => token,
            Some(_) => return Err(ClientBuildError::EmptyToken),
            None => return Err(ClientBuildError::MissingToken),
        };

        let user_agent = match self.user_agent {
            Some(user_agent) if !user_agent.trim().is_empty() => user_agent,
            Some(_) => return Err(ClientBuildError::EmptyUserAgent),
            None => String::from(USER_AGENT),
        };

        let client = reqwest::Client::builder()
            .user_agent(user_agent)
            .timeout(self.timeout)
            .build()
            .map_err(ClientBuildError::RequestClient)?;

        Ok(Http {
            client,
            token,
            base_url: self.base_url,
        })
    }
}

impl Http {
    pub fn builder() -> HttpBuilder {
        HttpBuilder::new()
    }

    pub async fn get_channel_messages(
        &self,
        channel_id: Snowflake,
        limit: Option<u8>,
    ) -> Result<Vec<Message>, ClientError> {
        let url = format!("{}/channels/{}/messages", self.base_url, channel_id.raw);
        let mut request = self.client.get(url).header("Authorization", &self.token);

        if let Some(limit) = limit {
            request = request.query(&[("limit", limit)]);
        }

        let response = request.send().await.map_err(ClientError::Request)?;
        let status = response.status();

        if !status.is_success() {
            let body = response.text().await.map_err(ClientError::Request)?;
            return Err(ClientError::Status { status, body });
        }

        response
            .json::<Vec<Message>>()
            .await
            .map_err(ClientError::Request)
    }
}

#[cfg(test)]
mod tests {
    use super::{ClientBuildError, Http};

    #[test]
    fn builder_requires_token() {
        let result = Http::builder().user_agent("Nitronanite/1.0").build();
        assert!(matches!(result, Err(ClientBuildError::MissingToken)));
    }

    #[test]
    fn builder_rejects_empty_fields() {
        let token_result = Http::builder()
            .token(" ")
            .user_agent("Nitronanite/1.0")
            .build();
        assert!(matches!(token_result, Err(ClientBuildError::EmptyToken)));

        let user_agent_result = Http::builder().token("abc").user_agent("").build();
        assert!(matches!(
            user_agent_result,
            Err(ClientBuildError::EmptyUserAgent)
        ));
    }

    #[test]
    fn builder_creates_client() {
        let client = Http::builder()
            .token("abc")
            .user_agent("Nitronanite/1.0")
            .build();

        assert!(client.is_ok());
    }
}
