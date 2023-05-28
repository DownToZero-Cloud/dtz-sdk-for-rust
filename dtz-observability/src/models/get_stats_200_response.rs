/*
 * DTZ Observability
 *
 * a generated client for the DTZ Observability API
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: jens@apimeister.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetStats200Response {
    #[serde(rename = "logCount", skip_serializing_if = "Option::is_none")]
    pub log_count: Option<i32>,
    #[serde(rename = "logSize", skip_serializing_if = "Option::is_none")]
    pub log_size: Option<i32>,
    #[serde(rename = "metricCount", skip_serializing_if = "Option::is_none")]
    pub metric_count: Option<i32>,
    #[serde(rename = "metricSize", skip_serializing_if = "Option::is_none")]
    pub metric_size: Option<i32>,
}

impl GetStats200Response {
    pub fn new() -> GetStats200Response {
        GetStats200Response {
            log_count: None,
            log_size: None,
            metric_count: None,
            metric_size: None,
        }
    }
}


