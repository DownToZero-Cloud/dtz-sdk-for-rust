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
pub struct QueryLogsRequest {
    #[serde(rename = "search", skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
}

impl QueryLogsRequest {
    pub fn new() -> QueryLogsRequest {
        QueryLogsRequest {
            search: None,
        }
    }
}


