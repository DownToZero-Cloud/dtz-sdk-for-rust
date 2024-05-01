/*
 * DTZ Core Api
 *
 * a generated client for the DTZ Core API
 *
 * The version of the OpenAPI document: 1.0.5
 * Contact: jens@apimeister.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContextResponse {
    #[serde(rename = "contextId")]
    pub context_id: uuid::Uuid,
    #[serde(rename = "created")]
    pub created: String,
    #[serde(rename = "alias", skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
}

impl ContextResponse {
    pub fn new(context_id: uuid::Uuid, created: String) -> ContextResponse {
        ContextResponse {
            context_id,
            created,
            alias: None,
        }
    }
}


