/*
 * DTZ Container Registry
 *
 * a generated client for the DTZ Container Registry API
 *
 * The version of the OpenAPI document: 1.0.0
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
    if base.host_str() == Some("localhost") || base.host_str() == Some("localhost6") {
        let _ = target_url.set_scheme(base.scheme());
        let _ = target_url.set_port(base.port());
        let _ = target_url.set_host(Some(base.host_str().unwrap()));
        format!("{target_url}")
    } else if base.scheme() == target_url.scheme() && base.host_str() == target_url.host_str() {
        format!("{target_url}")
    } else {
        let _ = target_url.set_scheme(base.scheme());
        let _ = target_url.set_port(base.port());
        let host_str = base.host_str().unwrap();
        let mut parts: Vec<&str> = host_str.split('.').collect();
        let target_str = target_url.host_str().unwrap();
        let target_parts: Vec<&str> = target_str.split('.').collect();
        parts.insert(0, target_parts.first().unwrap());
        let final_url = parts.join(".");
        let _ = target_url.set_host(Some(&final_url));
        format!("{target_url}")
    }
}


/// struct for typed errors of method [`check_authentication`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CheckAuthenticationError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`disable_service`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DisableServiceError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`enable_service`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EnableServiceError {
    UnknownValue(serde_json::Value),
}


/// check authentication
pub async fn check_authentication(configuration: &Configuration, ) -> Result<(), Error<CheckAuthenticationError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/", build_url(configuration));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        local_var_req_builder = local_var_req_builder.header("X-API-KEY", local_var_apikey);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<CheckAuthenticationError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: Some(crate::apis::Content::Text(local_var_content)), entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// disable the container registry service
pub async fn disable_service(configuration: &Configuration, ) -> Result<(), Error<DisableServiceError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/disable", build_url(configuration));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        local_var_req_builder = local_var_req_builder.header("X-API-KEY", local_var_apikey);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<DisableServiceError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: Some(crate::apis::Content::Text(local_var_content)), entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// enable the container registry service
pub async fn enable_service(configuration: &Configuration, ) -> Result<(), Error<EnableServiceError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/enable", build_url(configuration));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        local_var_req_builder = local_var_req_builder.header("X-API-KEY", local_var_apikey);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<EnableServiceError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: Some(crate::apis::Content::Text(local_var_content)), entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

