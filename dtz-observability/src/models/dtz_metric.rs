/*
 * DTZ Observability
 *
 * a generated client for the DTZ Observability API
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
pub struct DtzMetric {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "intervalStartTsNanos", skip_serializing_if = "Option::is_none")]
    pub interval_start_ts_nanos: Option<i64>,
    #[serde(rename = "intervalEndTsNanos")]
    pub interval_end_ts_nanos: i64,
    #[serde(rename = "value")]
    pub value: f64,
    #[serde(rename = "unit")]
    pub unit: String,
    #[serde(rename = "kind")]
    pub kind: Kind,
    #[serde(rename = "traceId", skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
    #[serde(rename = "spanId", skip_serializing_if = "Option::is_none")]
    pub span_id: Option<String>,
}

impl DtzMetric {
    pub fn new(name: String, interval_end_ts_nanos: i64, value: f64, unit: String, kind: Kind) -> DtzMetric {
        DtzMetric {
            name,
            description: None,
            attributes: None,
            interval_start_ts_nanos: None,
            interval_end_ts_nanos,
            value,
            unit,
            kind,
            trace_id: None,
            span_id: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Kind {
    #[serde(rename = "Gauge")]
    Gauge,
    #[serde(rename = "Sum")]
    Sum,
    #[serde(rename = "Counter")]
    Counter,
}

impl Default for Kind {
    fn default() -> Kind {
        Self::Gauge
    }
}

