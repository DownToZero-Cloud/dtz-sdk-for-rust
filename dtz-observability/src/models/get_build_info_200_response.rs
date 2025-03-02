/*
 * DTZ Observability
 *
 * a generated client for the DTZ Observability API
 *
 * The version of the OpenAPI document: 1.0.9
 * Contact: jens@apimeister.com
 * Generated by: https://openapi-generator.tech
 */

#[allow(unused)]
use crate::models;




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetBuildInfo200Response {
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Box<crate::models::GetBuildInfo200ResponseData>>,
}

impl GetBuildInfo200Response {
    pub fn new() -> GetBuildInfo200Response {
        GetBuildInfo200Response {
            status: None,
            data: None,
        }
    }
}


