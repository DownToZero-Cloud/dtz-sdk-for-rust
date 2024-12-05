/*
 * DTZ Billing Api
 *
 * a generated client for the DTZ Billing API
 *
 * The version of the OpenAPI document: 1.0.4
 * Contact: jens@apimeister.com
 * Generated by: https://openapi-generator.tech
 */

#[allow(unused)]
use crate::models;




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConsumptionValuesInner {
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<i32>,
    #[serde(rename = "unitOfAmount", skip_serializing_if = "Option::is_none")]
    pub unit_of_amount: Option<UnitOfAmount>,
}

impl ConsumptionValuesInner {
    pub fn new() -> ConsumptionValuesInner {
        ConsumptionValuesInner {
            description: None,
            amount: None,
            unit_of_amount: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UnitOfAmount {
    #[serde(rename = "GbSeconds")]
    GbSeconds,
    #[serde(rename = "Requests")]
    Requests,
    #[serde(rename = "MilliSeconds")]
    MilliSeconds,
    #[serde(rename = "Watt")]
    Watt,
    #[serde(rename = "Euro")]
    Euro,
}

impl Default for UnitOfAmount {
    fn default() -> UnitOfAmount {
        Self::GbSeconds
    }
}

