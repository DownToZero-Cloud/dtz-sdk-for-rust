/*
 * DTZ Core Api
 *
 * a generated client for the DTZ Core API
 *
 * Contact: jens@apimeister.com
 * Generated by: https://openapi-generator.tech
 */

#[allow(unused)]
use crate::models;




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateTaskRequestTaskDefinition {
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
}

impl CreateTaskRequestTaskDefinition {
    pub fn new(container_image: String) -> CreateTaskRequestTaskDefinition {
        CreateTaskRequestTaskDefinition {
            container_image,
            container_image_version: None,
            container_pull_user: None,
            container_pull_pwd: None,
            env_variables: None,
        }
    }
}


