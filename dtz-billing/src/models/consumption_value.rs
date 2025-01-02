/*
 * DTZ Billing Api
 *
 * a generated client for the DTZ Billing API
 *
 * The version of the OpenAPI document: 1.0.6
 * Contact: jens@apimeister.com
 * Generated by: https://openapi-generator.tech
 */

#[allow(unused)]
use crate::models;




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConsumptionValue {
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ecoMode", skip_serializing_if = "Option::is_none")]
    pub eco_mode: Option<bool>,
    #[serde(rename = "amount")]
    pub amount: f64,
    #[serde(rename = "unitOfAmount")]
    pub unit_of_amount: UnitOfAmount,
}

impl ConsumptionValue {
    pub fn new(amount: f64, unit_of_amount: UnitOfAmount) -> ConsumptionValue {
        ConsumptionValue {
            description: None,
            eco_mode: None,
            amount,
            unit_of_amount,
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
