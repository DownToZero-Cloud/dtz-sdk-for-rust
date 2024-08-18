/*
 * DTZ Core Api
 *
 * a generated client for the DTZ Core API
 *
 * The version of the OpenAPI document: 1.0.14
 * Contact: jens@apimeister.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
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


/// struct for typed errors of method [`context_context_id_enable_service_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContextContextIdEnableServiceGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_context`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateContextError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_ingress`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateIngressError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_root_ingress`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateRootIngressError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_task`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateTaskError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_context`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteContextError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_ingress`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteIngressError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_root_ingress`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteRootIngressError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_context`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetContextError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_current_context`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCurrentContextError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_ingress`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetIngressError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_root_ingress`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetRootIngressError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_task_history`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTaskHistoryError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`issue_certificate`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IssueCertificateError {
    Status429(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_ingress`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListIngressError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`pull_task_from_queue`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PullTaskFromQueueError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_context`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateContextError {
    UnknownValue(serde_json::Value),
}


pub async fn context_context_id_enable_service_get(configuration: &Configuration, context_id: &str) -> Result<(), Error<ContextContextIdEnableServiceGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/context/{context_id}/enableService", build_url(configuration), context_id=crate::apis::urlencode(context_id));
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
        let local_var_entity: Option<ContextContextIdEnableServiceGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: Some(crate::apis::Content::Text(local_var_content)), entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// create new context
pub async fn create_context(configuration: &Configuration, create_context_request: Option<crate::models::CreateContextRequest>) -> Result<models::ContextResponse, Error<CreateContextError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/context", build_url(configuration));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        local_var_req_builder = local_var_req_builder.header("X-API-KEY", local_var_apikey);
    };
    local_var_req_builder = local_var_req_builder.json(&create_context_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateContextError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: Some(crate::apis::Content::Text(local_var_content)), entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn create_ingress(configuration: &Configuration, domain: &str, uri: &str, create_ingress_request: Option<crate::models::CreateIngressRequest>) -> Result<models::IngressResponse, Error<CreateIngressError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/ingress/{domain}/{uri}", build_url(configuration), domain=crate::apis::urlencode(domain), uri=crate::apis::urlencode(uri));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        local_var_req_builder = local_var_req_builder.header("X-API-KEY", local_var_apikey);
    };
    local_var_req_builder = local_var_req_builder.json(&create_ingress_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateIngressError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: Some(crate::apis::Content::Text(local_var_content)), entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn create_root_ingress(configuration: &Configuration, domain: &str, create_ingress_request: Option<crate::models::CreateIngressRequest>) -> Result<models::IngressResponse, Error<CreateRootIngressError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/ingress/{domain}/", build_url(configuration), domain=crate::apis::urlencode(domain));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        local_var_req_builder = local_var_req_builder.header("X-API-KEY", local_var_apikey);
    };
    local_var_req_builder = local_var_req_builder.json(&create_ingress_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateRootIngressError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: Some(crate::apis::Content::Text(local_var_content)), entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn create_task(configuration: &Configuration, task_id: &str, create_task_request: Option<crate::models::CreateTaskRequest>) -> Result<(), Error<CreateTaskError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/task/{task_id}", build_url(configuration), task_id=crate::apis::urlencode(task_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        local_var_req_builder = local_var_req_builder.header("X-API-KEY", local_var_apikey);
    };
    local_var_req_builder = local_var_req_builder.json(&create_task_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<CreateTaskError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: Some(crate::apis::Content::Text(local_var_content)), entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn delete_context(configuration: &Configuration, context_id: &str) -> Result<(), Error<DeleteContextError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/context/{context_id}", build_url(configuration), context_id=crate::apis::urlencode(context_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

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
        let local_var_entity: Option<DeleteContextError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: Some(crate::apis::Content::Text(local_var_content)), entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn delete_ingress(configuration: &Configuration, domain: &str, uri: &str) -> Result<(), Error<DeleteIngressError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/ingress/{domain}/{uri}", build_url(configuration), domain=crate::apis::urlencode(domain), uri=crate::apis::urlencode(uri));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

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
        let local_var_entity: Option<DeleteIngressError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: Some(crate::apis::Content::Text(local_var_content)), entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn delete_root_ingress(configuration: &Configuration, domain: &str) -> Result<(), Error<DeleteRootIngressError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/ingress/{domain}/", build_url(configuration), domain=crate::apis::urlencode(domain));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

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
        let local_var_entity: Option<DeleteRootIngressError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: Some(crate::apis::Content::Text(local_var_content)), entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_context(configuration: &Configuration, context_id: &str) -> Result<models::ContextResponse, Error<GetContextError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/context/{context_id}", build_url(configuration), context_id=crate::apis::urlencode(context_id));
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
            serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetContextError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: Some(crate::apis::Content::Text(local_var_content)), entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// get current context
pub async fn get_current_context(configuration: &Configuration, ) -> Result<models::ContextResponse, Error<GetCurrentContextError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/context", build_url(configuration));
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
            serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetCurrentContextError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: Some(crate::apis::Content::Text(local_var_content)), entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_ingress(configuration: &Configuration, domain: &str, uri: &str) -> Result<models::IngressResponse, Error<GetIngressError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/ingress/{domain}/{uri}", build_url(configuration), domain=crate::apis::urlencode(domain), uri=crate::apis::urlencode(uri));
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
            serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetIngressError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: Some(crate::apis::Content::Text(local_var_content)), entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_root_ingress(configuration: &Configuration, domain: &str) -> Result<models::IngressResponse, Error<GetRootIngressError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/ingress/{domain}/", build_url(configuration), domain=crate::apis::urlencode(domain));
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
            serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetRootIngressError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: Some(crate::apis::Content::Text(local_var_content)), entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_task_history(configuration: &Configuration, task_id: &str) -> Result<(), Error<GetTaskHistoryError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/task/{task_id}", build_url(configuration), task_id=crate::apis::urlencode(task_id));
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
        let local_var_entity: Option<GetTaskHistoryError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: Some(crate::apis::Content::Text(local_var_content)), entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// issue a certificate -  if no certificate exists a new one is issued, if a certificate exists a new one will only be issued 3 days before its expiration 
pub async fn issue_certificate(configuration: &Configuration, issue_certificate_request: Option<crate::models::IssueCertificateRequest>) -> Result<(), Error<IssueCertificateError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/certificate", build_url(configuration));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        local_var_req_builder = local_var_req_builder.header("X-API-KEY", local_var_apikey);
    };
    local_var_req_builder = local_var_req_builder.json(&issue_certificate_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<IssueCertificateError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: Some(crate::apis::Content::Text(local_var_content)), entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn list_ingress(configuration: &Configuration, ) -> Result<Vec<models::IngressResponse>, Error<ListIngressError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/ingress", build_url(configuration));
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
            serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ListIngressError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: Some(crate::apis::Content::Text(local_var_content)), entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn pull_task_from_queue(configuration: &Configuration, pull_task_from_queue_request: Option<crate::models::PullTaskFromQueueRequest>) -> Result<models::PullTaskFromQueue200Response, Error<PullTaskFromQueueError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/task", build_url(configuration));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        local_var_req_builder = local_var_req_builder.header("X-API-KEY", local_var_apikey);
    };
    local_var_req_builder = local_var_req_builder.json(&pull_task_from_queue_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PullTaskFromQueueError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: Some(crate::apis::Content::Text(local_var_content)), entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// update context
pub async fn update_context(configuration: &Configuration, context_id: &str, create_context_request: Option<crate::models::CreateContextRequest>) -> Result<models::ContextResponse, Error<UpdateContextError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/context/{context_id}", build_url(configuration), context_id=crate::apis::urlencode(context_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        local_var_req_builder = local_var_req_builder.header("X-API-KEY", local_var_apikey);
    };
    local_var_req_builder = local_var_req_builder.json(&create_context_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateContextError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: Some(crate::apis::Content::Text(local_var_content)), entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

