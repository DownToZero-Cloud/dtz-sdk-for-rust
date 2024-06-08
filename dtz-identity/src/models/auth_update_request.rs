/*
 * DTZ Identity
 *
 * a generated client for the DTZ Identity API
 *
 * The version of the OpenAPI document: 1.0.6
 * Contact: jens@apimeister.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthUpdateRequest {
    #[serde(rename = "authentication_id", skip_serializing_if = "Option::is_none")]
    pub authentication_id: Option<String>,
}

impl AuthUpdateRequest {
    pub fn new() -> AuthUpdateRequest {
        AuthUpdateRequest {
            authentication_id: None,
        }
    }
}


