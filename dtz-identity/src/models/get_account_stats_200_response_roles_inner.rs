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
pub struct GetAccountStats200ResponseRolesInner {
    #[serde(rename = "contextId", skip_serializing_if = "Option::is_none")]
    pub context_id: Option<dtz_identifier::ContextId>,
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<f64>,
}

impl GetAccountStats200ResponseRolesInner {
    pub fn new() -> GetAccountStats200ResponseRolesInner {
        GetAccountStats200ResponseRolesInner {
            context_id: None,
            count: None,
        }
    }
}


