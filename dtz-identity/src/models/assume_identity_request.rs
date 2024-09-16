/*
 * DTZ Identity
 *
 * a generated client for the DTZ Identity API
 *
 * The version of the OpenAPI document: 1.0.14
 * Contact: jens@apimeister.com
 * Generated by: https://openapi-generator.tech
 */

#[allow(unused)]
use crate::models;




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssumeIdentityRequest {
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "identity_id", skip_serializing_if = "Option::is_none")]
    pub identity_id: Option<dtz_identifier::IdentityId>,
}

impl AssumeIdentityRequest {
    pub fn new() -> AssumeIdentityRequest {
        AssumeIdentityRequest {
            email: None,
            identity_id: None,
        }
    }
}


