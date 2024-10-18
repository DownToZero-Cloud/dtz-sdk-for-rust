/*
 * DTZ Identity
 *
 * a generated client for the DTZ Identity API
 *
 * The version of the OpenAPI document: 1.0.16
 * Contact: jens@apimeister.com
 * Generated by: https://openapi-generator.tech
 */

#[allow(unused)]
use crate::models;




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetAccountStats200Response {
    #[serde(rename = "roles", skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<models::GetAccountStats200ResponseRolesInner>>,
    #[serde(rename = "identityCount", skip_serializing_if = "Option::is_none")]
    pub identity_count: Option<f64>,
    #[serde(rename = "authenticationCount", skip_serializing_if = "Option::is_none")]
    pub authentication_count: Option<f64>,
}

impl GetAccountStats200Response {
    pub fn new() -> GetAccountStats200Response {
        GetAccountStats200Response {
            roles: None,
            identity_count: None,
            authentication_count: None,
        }
    }
}


