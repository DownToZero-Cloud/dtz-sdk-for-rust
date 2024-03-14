/*
 * DTZ RSS2Email Api
 *
 * a generated client for the DTZ RSS2Email API
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: jens@apimeister.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateFeed200Response {
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "lastCheck")]
    pub last_check: String,
    #[serde(rename = "lastDataFound")]
    pub last_data_found: String,
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[serde(rename = "name")]
    pub name: String,
}

impl CreateFeed200Response {
    pub fn new(id: uuid::Uuid, url: String, last_check: String, last_data_found: String, enabled: bool, name: String) -> CreateFeed200Response {
        CreateFeed200Response {
            id,
            url,
            last_check,
            last_data_found,
            enabled,
            name,
        }
    }
}


