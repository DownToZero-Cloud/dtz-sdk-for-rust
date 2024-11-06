/*
 * DTZ Billing Api
 *
 * a generated client for the DTZ Billing API
 *
 * The version of the OpenAPI document: 1.0.3
 * Contact: jens@apimeister.com
 * Generated by: https://openapi-generator.tech
 */

#[allow(unused)]
use crate::models;




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetStats200Response {
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    #[serde(rename = "ts", skip_serializing_if = "Option::is_none")]
    pub ts: Option<String>,
}

impl GetStats200Response {
    pub fn new() -> GetStats200Response {
        GetStats200Response {
            value: None,
            ts: None,
        }
    }
}


