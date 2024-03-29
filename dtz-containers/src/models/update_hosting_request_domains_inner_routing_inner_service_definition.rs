/*
 * DTZ Containers
 *
 * a generated client for the DTZ Containers API
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: jens@apimeister.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateHostingRequestDomainsInnerRoutingInnerServiceDefinition {
    #[serde(rename = "containerImage", skip_serializing_if = "Option::is_none")]
    pub container_image: Option<String>,
    #[serde(rename = "containerTag", skip_serializing_if = "Option::is_none")]
    pub container_tag: Option<String>,
    #[serde(rename = "containerPullUser", skip_serializing_if = "Option::is_none")]
    pub container_pull_user: Option<String>,
    #[serde(rename = "containerPullPwd", skip_serializing_if = "Option::is_none")]
    pub container_pull_pwd: Option<String>,
    #[serde(rename = "envVariables", skip_serializing_if = "Option::is_none")]
    pub env_variables: Option<serde_json::Value>,
}

impl UpdateHostingRequestDomainsInnerRoutingInnerServiceDefinition {
    pub fn new() -> UpdateHostingRequestDomainsInnerRoutingInnerServiceDefinition {
        UpdateHostingRequestDomainsInnerRoutingInnerServiceDefinition {
            container_image: None,
            container_tag: None,
            container_pull_user: None,
            container_pull_pwd: None,
            env_variables: None,
        }
    }
}


