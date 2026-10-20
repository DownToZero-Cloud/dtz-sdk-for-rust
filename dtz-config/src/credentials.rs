//! what a container fetches from the credential service of its launcher.
//!
//! inside the platform the launcher (dtz-blue for services, dtz-purple for
//! jobs) answers on the address in `DTZ_CREDENTIAL_URI`, and the container
//! proves who it is with the token file the launcher mounted for it. the
//! routes are `/token` for a fresh workload token of the container's
//! context, `/postgres-token` for the token of its database identity, and
//! `/secrets` for the parameters of the workload. a credential fetched here
//! can be fetched again, so a long-running container never holds an expired
//! one; a credential copied from the environment at start cannot

use std::{collections::HashMap, fmt, time::Duration};

/// the base of the dtz routes, set by the launcher
pub const URI_ENV: &str = "DTZ_CREDENTIAL_URI";
/// the token file the launcher mounted, set by the launcher
pub const TOKEN_FILE_ENV: &str = "DTZ_CREDENTIAL_TOKEN_FILE";
/// where the file is when the variable is not set
pub const DEFAULT_TOKEN_FILE: &str = "/var/run/dtz/credential-token";
/// the start-time workload token under the compatibility option
pub const ACCESS_TOKEN_ENV: &str = "DTZ_ACCESS_TOKEN";

#[derive(Debug)]
pub enum CredentialError {
    /// no credential service is configured, the process runs outside the platform
    NotInPlatform,
    /// the token file could not be read
    TokenFile(std::io::Error),
    Http(reqwest::Error),
    /// the service answered with an error status
    Refused(u16, String),
    /// the answer was not what the route promises
    Unreadable(String),
}

impl fmt::Display for CredentialError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotInPlatform => write!(f, "no credential service configured"),
            Self::TokenFile(err) => write!(f, "credential token file unreadable: {err}"),
            Self::Http(err) => write!(f, "credential service unreachable: {err}"),
            Self::Refused(status, body) => write!(f, "credential service refused: {status} {body}"),
            Self::Unreadable(what) => write!(f, "credential service answer unreadable: {what}"),
        }
    }
}

impl std::error::Error for CredentialError {}

impl From<reqwest::Error> for CredentialError {
    fn from(err: reqwest::Error) -> Self {
        Self::Http(err)
    }
}

/// the credential service of the launcher, as the environment names it
#[derive(Clone)]
pub struct CredentialService {
    base: String,
    token: String,
    client: reqwest::Client,
}

impl fmt::Debug for CredentialService {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CredentialService({})", self.base)
    }
}

#[derive(serde::Deserialize)]
struct TokenAnswer {
    token: String,
}

impl CredentialService {
    /// the service of this container; `NotInPlatform` without the address
    pub fn from_env() -> Result<Self, CredentialError> {
        let base = std::env::var(URI_ENV).map_err(|_| CredentialError::NotInPlatform)?;
        let file = std::env::var(TOKEN_FILE_ENV).unwrap_or_else(|_| DEFAULT_TOKEN_FILE.to_string());
        let token = std::fs::read_to_string(&file)
            .map_err(CredentialError::TokenFile)?
            .trim()
            .to_string();
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(5))
            .build()
            .unwrap_or_default();
        Ok(Self {
            base: base.trim_end_matches('/').to_string(),
            token,
            client,
        })
    }

    async fn get(&self, route: &str) -> Result<reqwest::Response, CredentialError> {
        let response = self
            .client
            .get(format!("{}/{route}", self.base))
            .header(reqwest::header::AUTHORIZATION, &self.token)
            .send()
            .await?;
        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(CredentialError::Refused(status.as_u16(), body));
        }
        Ok(response)
    }

    /// a fresh workload token of the container's context
    pub async fn workload_token(&self) -> Result<String, CredentialError> {
        let answer: TokenAnswer = self
            .get("token")
            .await?
            .json()
            .await
            .map_err(|err| CredentialError::Unreadable(err.to_string()))?;
        Ok(answer.token)
    }

    /// a fresh token of the container's database identity, the password of
    /// its database login
    pub async fn database_token(&self) -> Result<String, CredentialError> {
        let answer: TokenAnswer = self
            .get("postgres-token")
            .await?
            .json()
            .await
            .map_err(|err| CredentialError::Unreadable(err.to_string()))?;
        Ok(answer.token)
    }

    /// the parameters of the workload, name to value
    pub async fn secrets(&self) -> Result<HashMap<String, String>, CredentialError> {
        self.get("secrets")
            .await?
            .json()
            .await
            .map_err(|err| CredentialError::Unreadable(err.to_string()))
    }
}

/// the workload token of this container: from the credential service, and
/// where there is none, from `DTZ_ACCESS_TOKEN`, the start-time token of the
/// compatibility option. a service that calls this whenever it builds a
/// client never presents an expired token
pub async fn workload_token() -> Result<String, CredentialError> {
    match CredentialService::from_env() {
        Ok(service) => service.workload_token().await,
        Err(CredentialError::NotInPlatform) => std::env::var(ACCESS_TOKEN_ENV)
            .ok()
            .filter(|token| !token.is_empty())
            .ok_or(CredentialError::NotInPlatform),
        Err(err) => Err(err),
    }
}

impl crate::Configuration {
    /// a configuration that acts as the workload identity of this container,
    /// with a token fetched now
    pub async fn from_workload() -> Result<Self, CredentialError> {
        Ok(Self {
            oauth_access_token: Some(workload_token().await?),
            ..Default::default()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_service_prints_no_token() {
        let service = CredentialService {
            base: "http://169.254.170.23/dtz".to_string(),
            token: "secret".to_string(),
            client: reqwest::Client::new(),
        };
        assert_eq!(
            format!("{service:?}"),
            "CredentialService(http://169.254.170.23/dtz)"
        );
    }
}
