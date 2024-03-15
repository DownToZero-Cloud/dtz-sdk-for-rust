/*
 * DTZ RSS2Email Api
 *
 * a generated client for the DTZ RSS2Email API
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: jens@apimeister.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetProfile200Response {
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "subject", skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(rename = "stripeId", skip_serializing_if = "Option::is_none")]
    pub stripe_id: Option<String>,
    #[serde(rename = "contextId", skip_serializing_if = "Option::is_none")]
    pub context_id: Option<uuid::Uuid>,
    #[serde(rename = "identityId", skip_serializing_if = "Option::is_none")]
    pub identity_id: Option<uuid::Uuid>,
}

impl GetProfile200Response {
    pub fn new() -> GetProfile200Response {
        GetProfile200Response {
            email: None,
            subject: None,
            body: None,
            stripe_id: None,
            context_id: None,
            identity_id: None,
        }
    }
}

