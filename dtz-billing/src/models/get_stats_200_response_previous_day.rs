/*
 * DTZ Billing Api
 *
 * a generated client for the DTZ Billing API
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: jens@apimeister.com
 * Generated by: https://openapi-generator.tech
 */

#[allow(unused_imports)]
use crate::models;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetStats200ResponsePreviousDay {
    #[serde(rename = "euro", skip_serializing_if = "Option::is_none")]
    pub euro: Option<f64>,
    #[serde(rename = "watt", skip_serializing_if = "Option::is_none")]
    pub watt: Option<f64>,
}

impl GetStats200ResponsePreviousDay {
    pub fn new() -> GetStats200ResponsePreviousDay {
        GetStats200ResponsePreviousDay {
            euro: None,
            watt: None,
        }
    }
}

