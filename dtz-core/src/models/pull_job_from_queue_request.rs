/*
 * DTZ Core Api
 *
 * a generated client for the DTZ Core API
 *
 * The version of the OpenAPI document: 1.0.8
 * Contact: jens@apimeister.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PullJobFromQueueRequest {
    #[serde(rename = "nodeId")]
    pub node_id: String,
    #[serde(rename = "instanceId")]
    pub instance_id: String,
    #[serde(rename = "cpuCapacity")]
    pub cpu_capacity: i64,
    #[serde(rename = "memCapacity")]
    pub mem_capacity: i64,
    #[serde(rename = "ecoMode")]
    pub eco_mode: bool,
}

impl PullJobFromQueueRequest {
    pub fn new(node_id: String, instance_id: String, cpu_capacity: i64, mem_capacity: i64, eco_mode: bool) -> PullJobFromQueueRequest {
        PullJobFromQueueRequest {
            node_id,
            instance_id,
            cpu_capacity,
            mem_capacity,
            eco_mode,
        }
    }
}


