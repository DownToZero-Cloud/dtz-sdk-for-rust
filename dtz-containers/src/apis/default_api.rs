/*
 * DTZ Containers
 *
 * a generated client for the DTZ Containers API
 *
 * The version of the OpenAPI document: 1.0.10
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


/// struct for typed errors of method [`create_domain`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateDomainError {
    Status409(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_job`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateJobError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_service`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateServiceError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_domain`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteDomainError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_job`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteJobError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_service`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteServiceError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`disable`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DisableError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`enable`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EnableError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_domain`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDomainError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_domains`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDomainsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_job`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetJobError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_jobs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetJobsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_service`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetServiceError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_services`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetServicesError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`trigger_job`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TriggerJobError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_job`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateJobError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_service`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateServiceError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`verify_domain`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VerifyDomainError {
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}


pub async fn create_domain(configuration: &Configuration, create_domain: Option<crate::models::CreateDomain>) -> Result<models::Domain, Error<CreateDomainError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/domain", build_url(configuration));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        local_var_req_builder = local_var_req_builder.header("X-API-KEY", local_var_apikey);
    };
    local_var_req_builder = local_var_req_builder.json(&create_domain);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateDomainError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: Some(crate::apis::Content::Text(local_var_content)), entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn create_job(configuration: &Configuration, create_job_request: Option<crate::models::CreateJobRequest>) -> Result<(), Error<CreateJobError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/job", build_url(configuration));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        local_var_req_builder = local_var_req_builder.header("X-API-KEY", local_var_apikey);
    };
    local_var_req_builder = local_var_req_builder.json(&create_job_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<CreateJobError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: Some(crate::apis::Content::Text(local_var_content)), entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn create_service(configuration: &Configuration, create_service: Option<crate::models::CreateService>) -> Result<models::Service, Error<CreateServiceError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service", build_url(configuration));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        local_var_req_builder = local_var_req_builder.header("X-API-KEY", local_var_apikey);
    };
    local_var_req_builder = local_var_req_builder.json(&create_service);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateServiceError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: Some(crate::apis::Content::Text(local_var_content)), entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn delete_domain(configuration: &Configuration, domain_name: &str) -> Result<(), Error<DeleteDomainError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/domain/{domain_name}", build_url(configuration), domain_name=crate::apis::urlencode(domain_name));
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
        let local_var_entity: Option<DeleteDomainError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: Some(crate::apis::Content::Text(local_var_content)), entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn delete_job(configuration: &Configuration, job_id: &str) -> Result<(), Error<DeleteJobError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/job/{job_id}", build_url(configuration), job_id=crate::apis::urlencode(job_id));
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
        let local_var_entity: Option<DeleteJobError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: Some(crate::apis::Content::Text(local_var_content)), entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn delete_service(configuration: &Configuration, service_id: &str) -> Result<(), Error<DeleteServiceError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{serviceId}", build_url(configuration), serviceId=crate::apis::urlencode(service_id));
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
        let local_var_entity: Option<DeleteServiceError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: Some(crate::apis::Content::Text(local_var_content)), entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn disable(configuration: &Configuration, ) -> Result<(), Error<DisableError>> {
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
        let local_var_entity: Option<DisableError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: Some(crate::apis::Content::Text(local_var_content)), entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn enable(configuration: &Configuration, ) -> Result<(), Error<EnableError>> {
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
        let local_var_entity: Option<EnableError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: Some(crate::apis::Content::Text(local_var_content)), entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_domain(configuration: &Configuration, domain_name: &str) -> Result<models::Domain, Error<GetDomainError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/domain/{domain_name}", build_url(configuration), domain_name=crate::apis::urlencode(domain_name));
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
        let local_var_entity: Option<GetDomainError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: Some(crate::apis::Content::Text(local_var_content)), entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_domains(configuration: &Configuration, ) -> Result<Vec<models::Domain>, Error<GetDomainsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/domain", build_url(configuration));
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
        let local_var_entity: Option<GetDomainsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: Some(crate::apis::Content::Text(local_var_content)), entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_job(configuration: &Configuration, job_id: &str) -> Result<models::JobResponse, Error<GetJobError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/job/{job_id}", build_url(configuration), job_id=crate::apis::urlencode(job_id));
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
        let local_var_entity: Option<GetJobError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: Some(crate::apis::Content::Text(local_var_content)), entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_jobs(configuration: &Configuration, ) -> Result<Vec<models::GetJobs200ResponseInner>, Error<GetJobsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/job", build_url(configuration));
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
        let local_var_entity: Option<GetJobsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: Some(crate::apis::Content::Text(local_var_content)), entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_service(configuration: &Configuration, service_id: &str) -> Result<models::Service, Error<GetServiceError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{serviceId}", build_url(configuration), serviceId=crate::apis::urlencode(service_id));
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
        let local_var_entity: Option<GetServiceError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: Some(crate::apis::Content::Text(local_var_content)), entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_services(configuration: &Configuration, ) -> Result<Vec<models::Service>, Error<GetServicesError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service", build_url(configuration));
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
        let local_var_entity: Option<GetServicesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: Some(crate::apis::Content::Text(local_var_content)), entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn trigger_job(configuration: &Configuration, job_id: &str) -> Result<(), Error<TriggerJobError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/job/{job_id}", build_url(configuration), job_id=crate::apis::urlencode(job_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

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
        let local_var_entity: Option<TriggerJobError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: Some(crate::apis::Content::Text(local_var_content)), entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn update_job(configuration: &Configuration, job_id: &str) -> Result<models::JobResponse, Error<UpdateJobError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/job/{job_id}", build_url(configuration), job_id=crate::apis::urlencode(job_id));
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
            serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateJobError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: Some(crate::apis::Content::Text(local_var_content)), entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn update_service(configuration: &Configuration, service_id: &str, update_service_request: Option<crate::models::UpdateServiceRequest>) -> Result<models::Service, Error<UpdateServiceError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{serviceId}", build_url(configuration), serviceId=crate::apis::urlencode(service_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        local_var_req_builder = local_var_req_builder.header("X-API-KEY", local_var_apikey);
    };
    local_var_req_builder = local_var_req_builder.json(&update_service_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateServiceError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: Some(crate::apis::Content::Text(local_var_content)), entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn verify_domain(configuration: &Configuration, domain_name: &str) -> Result<(), Error<VerifyDomainError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/domain/{domain_name}", build_url(configuration), domain_name=crate::apis::urlencode(domain_name));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

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
        let local_var_entity: Option<VerifyDomainError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: Some(crate::apis::Content::Text(local_var_content)), entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

