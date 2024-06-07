/*
 * DTZ Identity
 *
 * a generated client for the DTZ Identity API
 *
 * The version of the OpenAPI document: 1.0.4
 * Contact: jens@apimeister.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeContextRequest {
    #[serde(rename = "contextId", skip_serializing_if = "Option::is_none")]
    pub context_id: Option<uuid::Uuid>,
}

impl ChangeContextRequest {
    pub fn new() -> ChangeContextRequest {
        ChangeContextRequest {
            context_id: None,
        }
    }
}


