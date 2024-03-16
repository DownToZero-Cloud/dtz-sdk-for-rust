/*
 * Dtz Objectstore
 *
 * a generated client for the DTZ Objectstore API
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: jens@apimeister.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use crate::models;
use super::Error;
use dtz::Configuration;

fn build_url(config: &dtz::Configuration) -> String {
    let base = url::Url::parse(&config.base_path).unwrap();
    let svc = url::Url::parse(crate::apis::configuration::SVC_URL).unwrap();
    let mut target_url = svc.clone();
    if base.host_str() == Some("localhost") || base.host_str() == Some("localhost6") {
        let _ = target_url.set_scheme(base.scheme());
        let _ = target_url.set_port(base.port());
        let _ = target_url.set_host(Some(base.host_str().unwrap()));
        format!("{target_url}")
    } else {
        if base.scheme() == svc.scheme() && base.host_str() == svc.host_str() {
            format!("{target_url}")
        } else {
            let _ = target_url.set_scheme(base.scheme());
            let _ = target_url.set_port(base.port());
            let host_str = base.host_str().unwrap();
            let mut parts: Vec<&str> = host_str.split('.').collect();
            let target_str = target_url.host_str().unwrap();
            let target_parts: Vec<&str> = target_str.split(".").collect();
            parts.insert(0, target_parts.first().unwrap());
            let final_url = parts.join(".");
            let _ = target_url.set_host(Some(&final_url));
            format!("{target_url}")
        }
    }
}


/// struct for typed errors of method [`delete_object`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteObjectError {
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

/// struct for typed errors of method [`get_object`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetObjectError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_objects`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListObjectsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`put_object`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PutObjectError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`stats`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StatsError {
    UnknownValue(serde_json::Value),
}


/// This can only be done by the logged in user.
pub async fn delete_object(configuration: &Configuration, object_path: &str) -> Result<models::Object, Error<DeleteObjectError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/obj/{objectPath}", build_url(&configuration), objectPath=crate::apis::urlencode(object_path));
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
            serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DeleteObjectError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: Some(crate::apis::Content::Text(local_var_content)), entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn disable_service(configuration: &Configuration, ) -> Result<(), Error<DisableServiceError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/disable", build_url(&configuration));
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
        let local_var_entity: Option<DisableServiceError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: Some(crate::apis::Content::Text(local_var_content)), entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn enable_service(configuration: &Configuration, ) -> Result<(), Error<EnableServiceError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/enable", build_url(&configuration));
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
        let local_var_entity: Option<EnableServiceError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: Some(crate::apis::Content::Text(local_var_content)), entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_object(configuration: &Configuration, object_path: &str) -> Result<Vec<u8>, Error<GetObjectError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/obj/{objectPath}", build_url(&configuration), objectPath=crate::apis::urlencode(object_path));
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
    let local_var_content = local_var_resp.bytes().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            Ok(local_var_content.to_vec())
    } else {
        let local_var_entity: Option<GetObjectError> = serde_json::from_slice(&local_var_content.to_vec()).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: Some(crate::apis::Content::Binary(local_var_content.to_vec())),
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn list_objects(configuration: &Configuration, ) -> Result<(), Error<ListObjectsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/obj/", build_url(&configuration));
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
        let local_var_entity: Option<ListObjectsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: Some(crate::apis::Content::Text(local_var_content)), entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn put_object(configuration: &Configuration, object_path: &str, body: Option<Vec<u8>>) -> Result<(), Error<PutObjectError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/obj/{objectPath}", build_url(&configuration), objectPath=crate::apis::urlencode(object_path));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        local_var_req_builder = local_var_req_builder.header("X-API-KEY", local_var_apikey);
    };
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(x) = body {
        local_var_req_builder = local_var_req_builder.body(x);
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<PutObjectError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: Some(crate::apis::Content::Text(local_var_content)), entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn stats(configuration: &Configuration, ) -> Result<models::Stats200Response, Error<StatsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/stats", build_url(&configuration));
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
        let local_var_entity: Option<StatsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: Some(crate::apis::Content::Text(local_var_content)), entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

