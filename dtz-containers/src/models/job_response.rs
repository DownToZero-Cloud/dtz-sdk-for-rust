/*
 * DTZ Containers
 *
 * a generated client for the DTZ Containers API
 *
 * The version of the OpenAPI document: 1.0.7
 * Contact: jens@apimeister.com
 * Generated by: https://openapi-generator.tech
 */

#[allow(unused)]
use crate::models;




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobResponse {
    #[serde(rename = "jobId")]
    pub job_id: dtz_identifier::JobId,
    #[serde(rename = "jobName")]
    pub job_name: String,
    #[serde(rename = "jobImage")]
    pub job_image: String,
    #[serde(rename = "jobPullUser", skip_serializing_if = "Option::is_none")]
    pub job_pull_user: Option<String>,
    #[serde(rename = "jobPullPassword", skip_serializing_if = "Option::is_none")]
    pub job_pull_password: Option<String>,
    #[serde(rename = "scheduleType")]
    pub schedule_type: String,
    #[serde(rename = "scheduleRepeat", skip_serializing_if = "Option::is_none")]
    pub schedule_repeat: Option<String>,
    #[serde(rename = "scheduleCron", skip_serializing_if = "Option::is_none")]
    pub schedule_cron: Option<String>,
    #[serde(rename = "scheduleCostOptimzation", skip_serializing_if = "Option::is_none")]
    pub schedule_cost_optimzation: Option<String>,
}

impl JobResponse {
    pub fn new(job_id: dtz_identifier::JobId, job_name: String, job_image: String, schedule_type: String) -> JobResponse {
        JobResponse {
            job_id,
            job_name,
            job_image,
            job_pull_user: None,
            job_pull_password: None,
            schedule_type,
            schedule_repeat: None,
            schedule_cron: None,
            schedule_cost_optimzation: None,
        }
    }
}


