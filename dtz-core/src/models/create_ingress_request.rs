/*
 * DTZ Core Api
 *
 * a generated client for the DTZ Core API
 *
 * The version of the OpenAPI document: 1.1.1
 * Contact: jens@apimeister.com
 * Generated by: https://openapi-generator.tech
 */

#[allow(unused)]
use crate::models;




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateIngressRequest {
    /// set the scope for this ingress, changes can only be performed within the same scope
    #[serde(rename = "scope")]
    pub scope: String,
    #[serde(rename = "validity", skip_serializing_if = "Option::is_none")]
    pub validity: Option<Box<crate::models::Validity>>,
    #[serde(rename = "login", skip_serializing_if = "Option::is_none")]
    pub login: Option<Box<crate::models::Login>>,
    #[serde(rename = "rewrite", skip_serializing_if = "Option::is_none")]
    pub rewrite: Option<Box<crate::models::Rewrite>>,
    #[serde(rename = "container", skip_serializing_if = "Option::is_none")]
    pub container: Option<Box<crate::models::Container>>,
    #[serde(rename = "staticContent", skip_serializing_if = "Option::is_none")]
    pub static_content: Option<Box<crate::models::StaticContent>>,
}

impl CreateIngressRequest {
    pub fn new(scope: String) -> CreateIngressRequest {
        CreateIngressRequest {
            scope,
            validity: None,
            login: None,
            rewrite: None,
            container: None,
            static_content: None,
        }
    }
}


