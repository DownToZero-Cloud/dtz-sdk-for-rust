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
pub struct ListAuthentication200Response {
    #[serde(rename = "identityId", skip_serializing_if = "Option::is_none")]
    pub identity_id: Option<dtz_identifier::IdentityId>,
    #[serde(rename = "userAuth", skip_serializing_if = "Option::is_none")]
    pub user_auth: Option<Vec<serde_json::Value>>,
    #[serde(rename = "apiKeyAuth", skip_serializing_if = "Option::is_none")]
    pub api_key_auth: Option<Vec<models::ListAuthentication200ResponseApiKeyAuthInner>>,
    #[serde(rename = "oauthAuth", skip_serializing_if = "Option::is_none")]
    pub oauth_auth: Option<Vec<serde_json::Value>>,
}

impl ListAuthentication200Response {
    pub fn new() -> ListAuthentication200Response {
        ListAuthentication200Response {
            identity_id: None,
            user_auth: None,
            api_key_auth: None,
            oauth_auth: None,
        }
    }
}


