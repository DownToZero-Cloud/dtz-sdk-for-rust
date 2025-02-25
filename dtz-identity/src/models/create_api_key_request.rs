/*
 * DTZ Identity
 *
 * a generated client for the DTZ Identity API
 *
 * The version of the OpenAPI document: 1.0.20
 * Contact: jens@apimeister.com
 * Generated by: https://openapi-generator.tech
 */

#[allow(unused)]
use crate::models;




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateApiKeyRequest {
    #[serde(rename = "contextId")]
    pub context_id: dtz_identifier::ContextId,
    #[serde(rename = "alias", skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
}

impl CreateApiKeyRequest {
    pub fn new(context_id: dtz_identifier::ContextId) -> CreateApiKeyRequest {
        CreateApiKeyRequest {
            context_id,
            alias: None,
        }
    }
}


