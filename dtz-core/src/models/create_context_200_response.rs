/*
 * DTZ Core Api
 *
 * a generated client for the DTZ Core API
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: jens@apimeister.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateContext200Response {
    #[serde(rename = "contextId", skip_serializing_if = "Option::is_none")]
    pub context_id: Option<String>,
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(rename = "alias", skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
}

impl CreateContext200Response {
    pub fn new() -> CreateContext200Response {
        CreateContext200Response {
            context_id: None,
            created: None,
            alias: None,
        }
    }
}

