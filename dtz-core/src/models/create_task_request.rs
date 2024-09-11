/*
 * DTZ Core Api
 *
 * a generated client for the DTZ Core API
 *
 * The version of the OpenAPI document: 1.0.15
 * Contact: jens@apimeister.com
 * Generated by: https://openapi-generator.tech
 */

#[allow(unused)]
use crate::models;




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateTaskRequest {
    #[serde(rename = "taskName")]
    pub task_name: String,
    /// origin service, like dtz-flows, dtz-containers
    #[serde(rename = "service")]
    pub service: String,
    #[serde(rename = "earliestStart")]
    pub earliest_start: String,
    #[serde(rename = "latestStart")]
    pub latest_start: String,
    #[serde(rename = "requireEcoMode")]
    pub require_eco_mode: bool,
    #[serde(rename = "taskDefinition")]
    pub task_definition: Box<crate::models::CreateTaskRequestTaskDefinition>,
}

impl CreateTaskRequest {
    pub fn new(task_name: String, service: String, earliest_start: String, latest_start: String, require_eco_mode: bool, task_definition: crate::models::CreateTaskRequestTaskDefinition) -> CreateTaskRequest {
        CreateTaskRequest {
            task_name,
            service,
            earliest_start,
            latest_start,
            require_eco_mode,
            task_definition: Box::new(task_definition),
        }
    }
}


