/*
 * DTZ Core Api
 *
 * a generated client for the DTZ Core API
 *
 * The version of the OpenAPI document: 1.0.1
 * Contact: jens@apimeister.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IngressResponseInner {
    #[serde(rename = "domain")]
    pub domain: String,
    #[serde(rename = "pathPrefix")]
    pub path_prefix: String,
    #[serde(rename = "container", skip_serializing_if = "Option::is_none")]
    pub container: Option<Box<crate::models::CreateIngressRequestContainer>>,
    #[serde(rename = "staticContent", skip_serializing_if = "Option::is_none")]
    pub static_content: Option<Box<crate::models::CreateIngressRequestStaticContent>>,
}

impl IngressResponseInner {
    pub fn new(domain: String, path_prefix: String) -> IngressResponseInner {
        IngressResponseInner {
            domain,
            path_prefix,
            container: None,
            static_content: None,
        }
    }
}


