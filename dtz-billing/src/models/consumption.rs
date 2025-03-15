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
pub struct Consumption {
    #[serde(rename = "service")]
    pub service: String,
    #[serde(rename = "contextId")]
    pub context_id: dtz_identifier::ContextId,
    #[serde(rename = "deduplicationKey")]
    pub deduplication_key: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "intervalStart")]
    pub interval_start: String,
    #[serde(rename = "intervalEnd")]
    pub interval_end: String,
    #[serde(rename = "values")]
    pub values: Vec<models::ConsumptionValue>,
}

impl Consumption {
    pub fn new(service: String, context_id: dtz_identifier::ContextId, deduplication_key: String, interval_start: String, interval_end: String, values: Vec<models::ConsumptionValue>) -> Consumption {
        Consumption {
            service,
            context_id,
            deduplication_key,
            description: None,
            interval_start,
            interval_end,
            values,
        }
    }
}


