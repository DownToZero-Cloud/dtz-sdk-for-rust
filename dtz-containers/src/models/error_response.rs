/*
 * DTZ Containers
 *
 * a generated client for the DTZ Containers API
 *
 * The version of the OpenAPI document: 1.0.3
 * Contact: jens@apimeister.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(rename = "msg", skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
}

impl ErrorResponse {
    pub fn new() -> ErrorResponse {
        ErrorResponse {
            msg: None,
        }
    }
}


