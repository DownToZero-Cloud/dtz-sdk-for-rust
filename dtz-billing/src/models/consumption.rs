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
pub struct Consumption {
    #[serde(rename = "service")]
    pub service: String,
    #[serde(rename = "contextId")]
    pub context_id: dtz_identifier::ContextId,
    #[serde(rename = "deduplicationKey", skip_serializing_if = "Option::is_none")]
    pub deduplication_key: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ts", skip_serializing_if = "Option::is_none")]
    pub ts: Option<String>,
    #[serde(rename = "amount")]
    pub amount: i32,
    #[serde(rename = "unitOfAmount")]
    pub unit_of_amount: UnitOfAmount,
    #[serde(rename = "amountMoney", skip_serializing_if = "Option::is_none")]
    pub amount_money: Option<f64>,
}

impl Consumption {
    pub fn new(service: String, context_id: dtz_identifier::ContextId, amount: i32, unit_of_amount: UnitOfAmount) -> Consumption {
        Consumption {
            service,
            context_id,
            deduplication_key: None,
            description: None,
            ts: None,
            amount,
            unit_of_amount,
            amount_money: None,
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
}

impl Default for UnitOfAmount {
    fn default() -> UnitOfAmount {
        Self::GbSeconds
    }
}

