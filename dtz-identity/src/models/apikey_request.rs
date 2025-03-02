/*
 * DTZ Identity
 *
 * a generated client for the DTZ Identity API
 *
 * The version of the OpenAPI document: 1.0.21
 * Contact: jens@apimeister.com
 * Generated by: https://openapi-generator.tech
 */

#[allow(unused)]
use crate::models;




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApikeyRequest {
    #[serde(rename = "apiKey")]
    pub api_key: String,
    #[serde(rename = "contextId", skip_serializing_if = "Option::is_none")]
    pub context_id: Option<dtz_identifier::ContextId>,
}

impl ApikeyRequest {
    pub fn new(api_key: String) -> ApikeyRequest {
        ApikeyRequest {
            api_key,
            context_id: None,
        }
    }
}


