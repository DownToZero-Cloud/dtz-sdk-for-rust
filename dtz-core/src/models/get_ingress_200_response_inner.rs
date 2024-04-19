/*
 * DTZ Core Api
 *
 * a generated client for the DTZ Core API
 *
 * The version of the OpenAPI document: 1.0.1
 * Contact: jens@apimeister.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetIngress200ResponseInner {
    #[serde(rename = "host")]
    pub host: String,
    #[serde(rename = "pathPrefix")]
    pub path_prefix: String,
    #[serde(rename = "containerImage")]
    pub container_image: String,
    #[serde(rename = "containerImageTag", skip_serializing_if = "Option::is_none")]
    pub container_image_tag: Option<String>,
    #[serde(rename = "containerImageVersion", skip_serializing_if = "Option::is_none")]
    pub container_image_version: Option<String>,
    #[serde(rename = "containerIdentity", skip_serializing_if = "Option::is_none")]
    pub container_identity: Option<String>,
    #[serde(rename = "containerPullUser", skip_serializing_if = "Option::is_none")]
    pub container_pull_user: Option<String>,
    #[serde(rename = "containerPullPwd", skip_serializing_if = "Option::is_none")]
    pub container_pull_pwd: Option<String>,
}

impl GetIngress200ResponseInner {
    pub fn new(host: String, path_prefix: String, container_image: String) -> GetIngress200ResponseInner {
        GetIngress200ResponseInner {
            host,
            path_prefix,
            container_image,
            container_image_tag: None,
            container_image_version: None,
            container_identity: None,
            container_pull_user: None,
            container_pull_pwd: None,
        }
    }
}


