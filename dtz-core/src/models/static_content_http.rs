/*
 * DTZ Core Api
 *
 * a generated client for the DTZ Core API
 *
 * The version of the OpenAPI document: 1.0.7
 * Contact: jens@apimeister.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StaticContentHttp {
    #[serde(rename = "header", skip_serializing_if = "Option::is_none")]
    pub header: Option<Vec<models::StaticContentHttpHeaderInner>>,
    #[serde(rename = "content")]
    pub content: String,
}

impl StaticContentHttp {
    pub fn new(content: String) -> StaticContentHttp {
        StaticContentHttp {
            header: None,
            content,
        }
    }
}


