/*
 * DTZ Containers
 *
 * a generated client for the DTZ Containers API
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: jens@apimeister.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateHostingRequest {
    #[serde(rename = "domains", skip_serializing_if = "Option::is_none")]
    pub domains: Option<Vec<models::UpdateHostingRequestDomainsInner>>,
}

impl UpdateHostingRequest {
    pub fn new() -> UpdateHostingRequest {
        UpdateHostingRequest {
            domains: None,
        }
    }
}


