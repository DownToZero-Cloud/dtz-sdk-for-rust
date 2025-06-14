/*
 * DTZ Identity
 *
 * a generated client for the DTZ Identity API
 *
 * Contact: jens@apimeister.com
 * Generated by: https://openapi-generator.tech
 */

#[allow(unused)]
use crate::models;




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssumeIdentityRequest {
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "identityId", skip_serializing_if = "Option::is_none")]
    pub identity_id: Option<dtz_identifier::IdentityId>,
    /// target context the token is issued for, if not present, a random context will be chosen.
    #[serde(rename = "contextId", skip_serializing_if = "Option::is_none")]
    pub context_id: Option<dtz_identifier::ContextId>,
}

impl AssumeIdentityRequest {
    pub fn new() -> AssumeIdentityRequest {
        AssumeIdentityRequest {
            email: None,
            identity_id: None,
            context_id: None,
        }
    }
}


