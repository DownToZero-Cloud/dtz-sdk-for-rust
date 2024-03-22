/*
 * DTZ Identity
 *
 * a generated client for the DTZ Identity API
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: jens@apimeister.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserSignup200Response {
    #[serde(rename = "access_token")]
    pub access_token: String,
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(rename = "token_type")]
    pub token_type: String,
    #[serde(rename = "expires_in")]
    pub expires_in: i32,
}

impl UserSignup200Response {
    pub fn new(access_token: String, token_type: String, expires_in: i32) -> UserSignup200Response {
        UserSignup200Response {
            access_token,
            scope: None,
            token_type,
            expires_in,
        }
    }
}

