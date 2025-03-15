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
pub struct GetStats200Response {
    #[serde(rename = "euro", skip_serializing_if = "Option::is_none")]
    pub euro: Option<Box<crate::models::GetStats200ResponseEuro>>,
    #[serde(rename = "watt", skip_serializing_if = "Option::is_none")]
    pub watt: Option<Box<crate::models::GetStats200ResponseEuro>>,
}

impl GetStats200Response {
    pub fn new() -> GetStats200Response {
        GetStats200Response {
            euro: None,
            watt: None,
        }
    }
}


