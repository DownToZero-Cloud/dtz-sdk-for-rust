/*
 * DTZ Core Api
 *
 * a generated client for the DTZ Core API
 *
 * The version of the OpenAPI document: 1.0.9
 * Contact: jens@apimeister.com
 * Generated by: https://openapi-generator.tech
 */

#[allow(unused)]
use crate::models;




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContextResponse {
    #[serde(rename = "contextId")]
    pub context_id: dtz_identifier::ContextId,
    #[serde(rename = "created")]
    pub created: String,
    #[serde(rename = "alias", skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
}

impl ContextResponse {
    pub fn new(context_id: dtz_identifier::ContextId, created: String) -> ContextResponse {
        ContextResponse {
            context_id,
            created,
            alias: None,
        }
    }
}


