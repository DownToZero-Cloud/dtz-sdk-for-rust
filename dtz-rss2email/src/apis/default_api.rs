/*
 * DTZ RSS2Email Api
 *
 * a generated client for the DTZ RSS2Email API
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: jens@apimeister.com
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::Error;
use crate::apis::ResponseContent;
use crate::models;
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

#[test]
fn ip() {
    let mut config = dtz::Configuration::default();
    config.base_path="http://localhost6:3000".to_string();
    let url = build_url(&config);
    println!("{url}");
}

/// struct for typed errors of method [`enable_service`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EnableServiceError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`rss2email_discover_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Rss2emailDiscoverPostError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`rss2email_feed_feed_id_delete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Rss2emailFeedFeedIdDeleteError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`rss2email_feed_feed_id_disable_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Rss2emailFeedFeedIdDisablePostError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`rss2email_feed_feed_id_enable_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Rss2emailFeedFeedIdEnablePostError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`rss2email_feed_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Rss2emailFeedGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`rss2email_feed_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Rss2emailFeedPostError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`rss2email_profile_cancel_subscription_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Rss2emailProfileCancelSubscriptionPostError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`rss2email_profile_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Rss2emailProfileGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`stats_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StatsGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_feed`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateFeedError {
    UnknownValue(serde_json::Value),
}

pub async fn enable_service(
    configuration: &Configuration,
) -> Result<(), Error<EnableServiceError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/enable", build_url(&configuration));
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

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
        let local_var_entity: Option<EnableServiceError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: Some(crate::apis::Content::Text(local_var_content)),
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn rss2email_discover_post(
    configuration: &Configuration,
    rss2email_discover_post_request: Option<crate::models::Rss2emailDiscoverPostRequest>,
) -> Result<models::Rss2emailDiscoverPost200Response, Error<Rss2emailDiscoverPostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/rss2email/discover", build_url(&configuration));
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        local_var_req_builder = local_var_req_builder.header("X-API-KEY", local_var_apikey);
    };
    local_var_req_builder = local_var_req_builder.json(&rss2email_discover_post_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<Rss2emailDiscoverPostError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: Some(crate::apis::Content::Text(local_var_content)),
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn rss2email_feed_feed_id_delete(
    configuration: &Configuration,
    feed_id: &str,
) -> Result<(), Error<Rss2emailFeedFeedIdDeleteError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/rss2email/feed/{feed_id}",
        build_url(&configuration),
        feed_id = crate::apis::urlencode(feed_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

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
        let local_var_entity: Option<Rss2emailFeedFeedIdDeleteError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: Some(crate::apis::Content::Text(local_var_content)),
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn rss2email_feed_feed_id_disable_post(
    configuration: &Configuration,
    feed_id: &str,
) -> Result<(), Error<Rss2emailFeedFeedIdDisablePostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/rss2email/feed/{feed_id}/disable",
        build_url(&configuration),
        feed_id = crate::apis::urlencode(feed_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

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
        let local_var_entity: Option<Rss2emailFeedFeedIdDisablePostError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: Some(crate::apis::Content::Text(local_var_content)),
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn rss2email_feed_feed_id_enable_post(
    configuration: &Configuration,
    feed_id: &str,
) -> Result<(), Error<Rss2emailFeedFeedIdEnablePostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/rss2email/feed/{feed_id}/enable",
        build_url(&configuration),
        feed_id = crate::apis::urlencode(feed_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

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
        let local_var_entity: Option<Rss2emailFeedFeedIdEnablePostError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: Some(crate::apis::Content::Text(local_var_content)),
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn rss2email_feed_get(
    configuration: &Configuration,
) -> Result<(), Error<Rss2emailFeedGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/rss2email/feed", build_url(&configuration));
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

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
        let local_var_entity: Option<Rss2emailFeedGetError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: Some(crate::apis::Content::Text(local_var_content)),
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn rss2email_feed_post(
    configuration: &Configuration,
    rss2email_feed_post_request: Option<crate::models::Rss2emailFeedPostRequest>,
) -> Result<models::Rss2emailFeedPost200Response, Error<Rss2emailFeedPostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/rss2email/feed", build_url(&configuration));
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        local_var_req_builder = local_var_req_builder.header("X-API-KEY", local_var_apikey);
    };
    local_var_req_builder = local_var_req_builder.json(&rss2email_feed_post_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<Rss2emailFeedPostError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: Some(crate::apis::Content::Text(local_var_content)),
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn rss2email_profile_cancel_subscription_post(
    configuration: &Configuration,
) -> Result<(), Error<Rss2emailProfileCancelSubscriptionPostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/rss2email/profile/cancelSubscription",
        build_url(&configuration)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

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
        let local_var_entity: Option<Rss2emailProfileCancelSubscriptionPostError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: Some(crate::apis::Content::Text(local_var_content)),
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn rss2email_profile_get(
    configuration: &Configuration,
) -> Result<models::Rss2emailProfileGet200Response, Error<Rss2emailProfileGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/rss2email/profile", build_url(&configuration));
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

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
        let local_var_entity: Option<Rss2emailProfileGetError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: Some(crate::apis::Content::Text(local_var_content)),
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn stats_get(configuration: &Configuration) -> Result<(), Error<StatsGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/stats", build_url(&configuration));
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

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
        let local_var_entity: Option<StatsGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: Some(crate::apis::Content::Text(local_var_content)),
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn update_feed(
    configuration: &Configuration,
    feed_id: &str,
    rss2email_feed_post200_response: Option<crate::models::Rss2emailFeedPost200Response>,
) -> Result<(), Error<UpdateFeedError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/rss2email/feed/{feed_id}",
        build_url(&configuration),
        feed_id = crate::apis::urlencode(feed_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        local_var_req_builder = local_var_req_builder.header("X-API-KEY", local_var_apikey);
    };
    local_var_req_builder = local_var_req_builder.json(&rss2email_feed_post200_response);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<UpdateFeedError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: Some(crate::apis::Content::Text(local_var_content)),
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
