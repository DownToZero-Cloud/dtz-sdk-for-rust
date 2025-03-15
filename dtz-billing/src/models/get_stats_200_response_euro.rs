/*
 * DTZ Billing Api
 *
 * a generated client for the DTZ Billing API
 *
 * Contact: jens@apimeister.com
 * Generated by: https://openapi-generator.tech
 */

#[allow(unused)]
use crate::models;




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetStats200ResponseEuro {
    #[serde(rename = "ts", skip_serializing_if = "Option::is_none")]
    pub ts: Option<String>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

impl GetStats200ResponseEuro {
    pub fn new() -> GetStats200ResponseEuro {
        GetStats200ResponseEuro {
            ts: None,
            value: None,
        }
    }
}


