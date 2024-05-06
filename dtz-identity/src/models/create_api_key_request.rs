/*
 * DTZ Identity
 *
 * a generated client for the DTZ Identity API
 *
 * The version of the OpenAPI document: 1.0.1
 * Contact: jens@apimeister.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateApiKeyRequest {
    #[serde(rename = "context_id", skip_serializing_if = "Option::is_none")]
    pub context_id: Option<uuid::Uuid>,
}

impl CreateApiKeyRequest {
    pub fn new() -> CreateApiKeyRequest {
        CreateApiKeyRequest {
            context_id: None,
        }
    }
}


