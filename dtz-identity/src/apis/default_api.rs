/*
 * DTZ Identity
 *
 * a generated client for the DTZ Identity API
 *
 * The version of the OpenAPI document: 1.0.17
 * Contact: jens@apimeister.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
#[allow(unused)]
use crate::models;
use super::Error;
use dtz_config::Configuration;

fn build_url(config: &Configuration) -> String {
    let base = url::Url::parse(&config.base_path).unwrap();
    let mut target_url = url::Url::parse(crate::apis::configuration::SVC_URL).unwrap();
    let _ = target_url.set_scheme(base.scheme());
    let _ = target_url.set_port(base.port());
    let _ = target_url.set_host(Some(base.host_str().unwrap()));
    format!("{target_url}")
}


/// struct for typed errors of method [`assign_role`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AssignRoleError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`assume_identity`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AssumeIdentityError {
    Status401(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`authenticate_apikey`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AuthenticateApikeyError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`change_authentication`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChangeAuthenticationError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_api_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateApiKeyError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_api_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteApiKeyError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_context_roles`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteContextRolesError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_identity`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteIdentityError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_account_email`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAccountEmailError {
    Status404(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_account_stats`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAccountStatsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_roles`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetRolesError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_authentication`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAuthenticationError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`new_context`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NewContextError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`oauth_authorize`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OauthAuthorizeError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`oauth_token`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OauthTokenError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`remove_role_assignment`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RemoveRoleAssignmentError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`token_refresh`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TokenRefreshError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`user_login`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UserLoginError {
    Status401(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`user_signup`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UserSignupError {
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}


pub async fn assign_role(configuration: &Configuration, role_id: &str) -> Result<(), Error<AssignRoleError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/me/roles/{roleId}", build_url(configuration), roleId=crate::apis::urlencode(role_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        local_var_req_builder = local_var_req_builder.header("X-API-KEY", local_var_apikey);
    };
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<AssignRoleError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: Some(crate::apis::Content::Text(local_var_content)), entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn assume_identity(configuration: &Configuration, assume_identity_request: Option<crate::models::AssumeIdentityRequest>) -> Result<models::TokenResponse, Error<AssumeIdentityError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/identity/assume", build_url(configuration));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        local_var_req_builder = local_var_req_builder.header("X-API-KEY", local_var_apikey);
    };
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&assume_identity_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AssumeIdentityError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: Some(crate::apis::Content::Text(local_var_content)), entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn authenticate_apikey(configuration: &Configuration, apikey_request: Option<crate::models::ApikeyRequest>) -> Result<models::TokenResponse, Error<AuthenticateApikeyError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/auth/apikey", build_url(configuration));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        local_var_req_builder = local_var_req_builder.header("X-API-KEY", local_var_apikey);
    };
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&apikey_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AuthenticateApikeyError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: Some(crate::apis::Content::Text(local_var_content)), entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn change_authentication(configuration: &Configuration, change_authentication_request: Option<crate::models::ChangeAuthenticationRequest>) -> Result<(), Error<ChangeAuthenticationError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/authentication", build_url(configuration));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        local_var_req_builder = local_var_req_builder.header("X-API-KEY", local_var_apikey);
    };
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&change_authentication_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<ChangeAuthenticationError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: Some(crate::apis::Content::Text(local_var_content)), entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn create_api_key(configuration: &Configuration, create_api_key_request: crate::models::CreateApiKeyRequest) -> Result<String, Error<CreateApiKeyError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/me/identity/apikey", build_url(configuration));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        local_var_req_builder = local_var_req_builder.header("X-API-KEY", local_var_apikey);
    };
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&create_api_key_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateApiKeyError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: Some(crate::apis::Content::Text(local_var_content)), entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn delete_api_key(configuration: &Configuration, apikey: &str) -> Result<(), Error<DeleteApiKeyError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/me/identity/apikey/{apikey}", build_url(configuration), apikey=crate::apis::urlencode(apikey));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        local_var_req_builder = local_var_req_builder.header("X-API-KEY", local_var_apikey);
    };
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<DeleteApiKeyError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: Some(crate::apis::Content::Text(local_var_content)), entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn delete_context_roles(configuration: &Configuration, context_id: &str) -> Result<(), Error<DeleteContextRolesError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/context/{context_id}", build_url(configuration), context_id=crate::apis::urlencode(context_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        local_var_req_builder = local_var_req_builder.header("X-API-KEY", local_var_apikey);
    };
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<DeleteContextRolesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: Some(crate::apis::Content::Text(local_var_content)), entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn delete_identity(configuration: &Configuration, ) -> Result<(), Error<DeleteIdentityError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/me/identity", build_url(configuration));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        local_var_req_builder = local_var_req_builder.header("X-API-KEY", local_var_apikey);
    };
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<DeleteIdentityError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: Some(crate::apis::Content::Text(local_var_content)), entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_account_email(configuration: &Configuration, ) -> Result<models::GetAccountEmail200Response, Error<GetAccountEmailError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/me/email", build_url(configuration));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        local_var_req_builder = local_var_req_builder.header("X-API-KEY", local_var_apikey);
    };
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetAccountEmailError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: Some(crate::apis::Content::Text(local_var_content)), entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_account_stats(configuration: &Configuration, ) -> Result<models::GetAccountStats200Response, Error<GetAccountStatsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/me", build_url(configuration));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        local_var_req_builder = local_var_req_builder.header("X-API-KEY", local_var_apikey);
    };
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetAccountStatsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: Some(crate::apis::Content::Text(local_var_content)), entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_roles(configuration: &Configuration, ) -> Result<models::GetRoles200Response, Error<GetRolesError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/roles", build_url(configuration));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        local_var_req_builder = local_var_req_builder.header("X-API-KEY", local_var_apikey);
    };
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetRolesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: Some(crate::apis::Content::Text(local_var_content)), entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn list_authentication(configuration: &Configuration, ) -> Result<(), Error<ListAuthenticationError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/authentication", build_url(configuration));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        local_var_req_builder = local_var_req_builder.header("X-API-KEY", local_var_apikey);
    };
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<ListAuthenticationError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: Some(crate::apis::Content::Text(local_var_content)), entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn new_context(configuration: &Configuration, context_id: &str, new_context_request: Option<crate::models::NewContextRequest>) -> Result<(), Error<NewContextError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/context/{context_id}/new", build_url(configuration), context_id=crate::apis::urlencode(context_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        local_var_req_builder = local_var_req_builder.header("X-API-KEY", local_var_apikey);
    };
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&new_context_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<NewContextError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: Some(crate::apis::Content::Text(local_var_content)), entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn oauth_authorize(configuration: &Configuration, response_type: &str, client_id: &str, redirect_uri: &str, scope: &str, state: Option<&str>, nonce: Option<&str>) -> Result<reqwest::header::HeaderMap, Error<OauthAuthorizeError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/oauth/authorize", build_url(configuration));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("response_type", &response_type.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("client_id", &client_id.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("redirect_uri", &redirect_uri.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("scope", &scope.to_string())]);
    if let Some(ref local_var_str) = state {
        local_var_req_builder = local_var_req_builder.query(&[("state", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = nonce {
        local_var_req_builder = local_var_req_builder.query(&[("nonce", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        local_var_req_builder = local_var_req_builder.header("X-API-KEY", local_var_apikey);
    };
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_headers = local_var_resp.headers().clone();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(local_var_headers)
    } else {
        let local_var_entity: Option<OauthAuthorizeError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: Some(crate::apis::Content::Text(local_var_content)), entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// oauth token request
pub async fn oauth_token(configuration: &Configuration, grant_type: &str, client_id: &str, client_secret: &str, redirect_uri: &str, code: &str) -> Result<models::TokenResponse, Error<OauthTokenError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/oauth/token", build_url(configuration));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        local_var_req_builder = local_var_req_builder.header("X-API-KEY", local_var_apikey);
    };
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    let mut local_var_form_params = std::collections::HashMap::new();
    local_var_form_params.insert("grant_type", grant_type.to_string());
    local_var_form_params.insert("client_id", client_id.to_string());
    local_var_form_params.insert("client_secret", client_secret.to_string());
    local_var_form_params.insert("redirect_uri", redirect_uri.to_string());
    local_var_form_params.insert("code", code.to_string());
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<OauthTokenError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: Some(crate::apis::Content::Text(local_var_content)), entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn remove_role_assignment(configuration: &Configuration, role_id: &str) -> Result<(), Error<RemoveRoleAssignmentError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/me/roles/{roleId}", build_url(configuration), roleId=crate::apis::urlencode(role_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        local_var_req_builder = local_var_req_builder.header("X-API-KEY", local_var_apikey);
    };
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<RemoveRoleAssignmentError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: Some(crate::apis::Content::Text(local_var_content)), entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// token refresh
pub async fn token_refresh(configuration: &Configuration, change_context_request: crate::models::ChangeContextRequest) -> Result<models::TokenResponse, Error<TokenRefreshError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/token/refresh", build_url(configuration));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        local_var_req_builder = local_var_req_builder.header("X-API-KEY", local_var_apikey);
    };
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&change_context_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<TokenRefreshError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: Some(crate::apis::Content::Text(local_var_content)), entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn user_login(configuration: &Configuration, auth_request: crate::models::AuthRequest) -> Result<models::TokenResponse, Error<UserLoginError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/token/auth", build_url(configuration));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        local_var_req_builder = local_var_req_builder.header("X-API-KEY", local_var_apikey);
    };
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&auth_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UserLoginError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: Some(crate::apis::Content::Text(local_var_content)), entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn user_signup(configuration: &Configuration, signup_request: crate::models::SignupRequest) -> Result<models::TokenResponse, Error<UserSignupError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/signup", build_url(configuration));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        local_var_req_builder = local_var_req_builder.header("X-API-KEY", local_var_apikey);
    };
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&signup_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UserSignupError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: Some(crate::apis::Content::Text(local_var_content)), entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

