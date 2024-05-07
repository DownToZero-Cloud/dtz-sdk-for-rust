/*
 * DTZ Core Api
 *
 * a generated client for the DTZ Core API
 *
 * The version of the OpenAPI document: 1.0.6
 * Contact: jens@apimeister.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PullJobFromQueue200Response {
    #[serde(rename = "contextId")]
    pub context_id: uuid::Uuid,
    #[serde(rename = "executionId")]
    pub execution_id: uuid::Uuid,
    #[serde(rename = "jobId")]
    pub job_id: uuid::Uuid,
    #[serde(rename = "jobName")]
    pub job_name: String,
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

impl PullJobFromQueue200Response {
    pub fn new(context_id: uuid::Uuid, execution_id: uuid::Uuid, job_id: uuid::Uuid, job_name: String, container_image: String) -> PullJobFromQueue200Response {
        PullJobFromQueue200Response {
            context_id,
            execution_id,
            job_id,
            job_name,
            container_image,
            container_image_version: None,
            container_pull_user: None,
            container_pull_pwd: None,
            env_variables: None,
        }
    }
}


