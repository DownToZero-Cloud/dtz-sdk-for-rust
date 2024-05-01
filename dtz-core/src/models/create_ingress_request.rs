/*
 * DTZ Core Api
 *
 * a generated client for the DTZ Core API
 *
 * The version of the OpenAPI document: 1.0.5
 * Contact: jens@apimeister.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateIngressRequest {
    #[serde(rename = "container", skip_serializing_if = "Option::is_none")]
    pub container: Option<Box<crate::models::Container>>,
    #[serde(rename = "staticContent", skip_serializing_if = "Option::is_none")]
    pub static_content: Option<Box<crate::models::StaticContent>>,
}

impl CreateIngressRequest {
    pub fn new() -> CreateIngressRequest {
        CreateIngressRequest {
            container: None,
            static_content: None,
        }
    }
}


