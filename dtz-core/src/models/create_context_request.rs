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
pub struct CreateContextRequest {
    #[serde(rename = "alias", skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
}

impl CreateContextRequest {
    pub fn new() -> CreateContextRequest {
        CreateContextRequest {
            alias: None,
        }
    }
}


