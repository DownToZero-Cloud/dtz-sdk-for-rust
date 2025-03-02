/*
 * DTZ Observability
 *
 * a generated client for the DTZ Observability API
 *
 * The version of the OpenAPI document: 1.0.9
 * Contact: jens@apimeister.com
 * Generated by: https://openapi-generator.tech
 */

#[allow(unused)]
use crate::models;




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetLogAttributes200ResponseInner {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
}

impl GetLogAttributes200ResponseInner {
    pub fn new() -> GetLogAttributes200ResponseInner {
        GetLogAttributes200ResponseInner {
            name: None,
            count: None,
        }
    }
}


