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
pub struct NewContextRequest {
    #[serde(rename = "context_id", skip_serializing_if = "Option::is_none")]
    pub context_id: Option<uuid::Uuid>,
    #[serde(rename = "identity_id", skip_serializing_if = "Option::is_none")]
    pub identity_id: Option<uuid::Uuid>,
    #[serde(rename = "service_principal_id", skip_serializing_if = "Option::is_none")]
    pub service_principal_id: Option<uuid::Uuid>,
}

impl NewContextRequest {
    pub fn new() -> NewContextRequest {
        NewContextRequest {
            context_id: None,
            identity_id: None,
            service_principal_id: None,
        }
    }
}


