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
pub struct GetAllContext200Response {
    #[serde(rename = "contexts", skip_serializing_if = "Option::is_none")]
    pub contexts: Option<Vec<models::GetAllContext200ResponseContextsInner>>,
}

impl GetAllContext200Response {
    pub fn new() -> GetAllContext200Response {
        GetAllContext200Response {
            contexts: None,
        }
    }
}


