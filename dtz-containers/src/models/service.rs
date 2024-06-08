/*
 * DTZ Containers
 *
 * a generated client for the DTZ Containers API
 *
 * The version of the OpenAPI document: 1.0.5
 * Contact: jens@apimeister.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Service {
    #[serde(rename = "contextId")]
    pub context_id: uuid::Uuid,
    #[serde(rename = "serviceId")]
    pub service_id: uuid::Uuid,
    #[serde(rename = "created")]
    pub created: String,
    #[serde(rename = "prefix")]
    pub prefix: String,
    #[serde(rename = "containerImage")]
    pub container_image: String,
    #[serde(rename = "containerImageVersion", skip_serializing_if = "Option::is_none")]
    pub container_image_version: Option<String>,
    #[serde(rename = "containerPullUser", skip_serializing_if = "Option::is_none")]
    pub container_pull_user: Option<String>,
    #[serde(rename = "containerPullPwd", skip_serializing_if = "Option::is_none")]
    pub container_pull_pwd: Option<String>,
    #[serde(rename = "envVariables", skip_serializing_if = "Option::is_none")]
    pub env_variables: Option<serde_json::Value>,
    #[serde(rename = "login", skip_serializing_if = "Option::is_none")]
    pub login: Option<Box<crate::models::ServiceLogin>>,
}

impl Service {
    pub fn new(context_id: uuid::Uuid, service_id: uuid::Uuid, created: String, prefix: String, container_image: String) -> Service {
        Service {
            context_id,
            service_id,
            created,
            prefix,
            container_image,
            container_image_version: None,
            container_pull_user: None,
            container_pull_pwd: None,
            env_variables: None,
            login: None,
        }
    }
}


