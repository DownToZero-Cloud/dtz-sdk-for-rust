/*
 * DTZ Core Api
 *
 * a generated client for the DTZ Core API
 *
 * The version of the OpenAPI document: 1.0.14
 * Contact: jens@apimeister.com
 * Generated by: https://openapi-generator.tech
 */

#[allow(unused)]
use crate::models;




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StaticContent {
    #[serde(rename = "http", skip_serializing_if = "Option::is_none")]
    pub http: Option<Box<crate::models::StaticContentHttp>>,
    #[serde(rename = "https", skip_serializing_if = "Option::is_none")]
    pub https: Option<Box<crate::models::StaticContentHttp>>,
}

impl StaticContent {
    pub fn new() -> StaticContent {
        StaticContent {
            http: None,
            https: None,
        }
    }
}


