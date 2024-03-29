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
pub struct UpdateHostingRequestDomainsInnerRoutingInner {
    #[serde(rename = "prefix", skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "serviceDefinition", skip_serializing_if = "Option::is_none")]
    pub service_definition: Option<Box<crate::models::UpdateHostingRequestDomainsInnerRoutingInnerServiceDefinition>>,
}

impl UpdateHostingRequestDomainsInnerRoutingInner {
    pub fn new() -> UpdateHostingRequestDomainsInnerRoutingInner {
        UpdateHostingRequestDomainsInnerRoutingInner {
            prefix: None,
            service_definition: None,
        }
    }
}


