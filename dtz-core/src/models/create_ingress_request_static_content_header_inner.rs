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
pub struct CreateIngressRequestStaticContentHeaderInner {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl CreateIngressRequestStaticContentHeaderInner {
    pub fn new() -> CreateIngressRequestStaticContentHeaderInner {
        CreateIngressRequestStaticContentHeaderInner {
            name: None,
            value: None,
        }
    }
}


