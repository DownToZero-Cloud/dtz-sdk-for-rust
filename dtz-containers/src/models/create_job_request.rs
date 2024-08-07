/*
 * DTZ Containers
 *
 * a generated client for the DTZ Containers API
 *
 * The version of the OpenAPI document: 1.0.9
 * Contact: jens@apimeister.com
 * Generated by: https://openapi-generator.tech
 */

#[allow(unused)]
use crate::models;




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateJobRequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "image", skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(rename = "pullUser", skip_serializing_if = "Option::is_none")]
    pub pull_user: Option<String>,
    #[serde(rename = "pullPwd", skip_serializing_if = "Option::is_none")]
    pub pull_pwd: Option<String>,
    #[serde(rename = "scheduleType", skip_serializing_if = "Option::is_none")]
    pub schedule_type: Option<String>,
    #[serde(rename = "scheduleCron", skip_serializing_if = "Option::is_none")]
    pub schedule_cron: Option<String>,
    #[serde(rename = "scheduleCostOptimzation", skip_serializing_if = "Option::is_none")]
    pub schedule_cost_optimzation: Option<String>,
    #[serde(rename = "scheduleRepeat", skip_serializing_if = "Option::is_none")]
    pub schedule_repeat: Option<String>,
}

impl CreateJobRequest {
    pub fn new() -> CreateJobRequest {
        CreateJobRequest {
            name: None,
            image: None,
            pull_user: None,
            pull_pwd: None,
            schedule_type: None,
            schedule_cron: None,
            schedule_cost_optimzation: None,
            schedule_repeat: None,
        }
    }
}


