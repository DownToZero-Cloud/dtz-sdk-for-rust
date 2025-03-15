/*
 * DTZ Observability
 *
 * a generated client for the DTZ Observability API
 *
 * Contact: jens@apimeister.com
 * Generated by: https://openapi-generator.tech
 */

#[allow(unused)]
use crate::models;




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetLogActivity200ResponseInner {
    #[serde(rename = "ts", skip_serializing_if = "Option::is_none")]
    pub ts: Option<String>,
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
}

impl GetLogActivity200ResponseInner {
    pub fn new() -> GetLogActivity200ResponseInner {
        GetLogActivity200ResponseInner {
            ts: None,
            count: None,
        }
    }
}


