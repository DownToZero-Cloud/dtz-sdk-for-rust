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
pub struct GetJobs200ResponseInner {
    #[serde(rename = "jobId")]
    pub job_id: String,
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

impl GetJobs200ResponseInner {
    pub fn new(job_id: String, job_name: String, job_image: String, schedule_type: String) -> GetJobs200ResponseInner {
        GetJobs200ResponseInner {
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


