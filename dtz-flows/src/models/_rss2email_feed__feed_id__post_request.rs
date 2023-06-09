/*
 * DTZ Flows Api
 *
 * a generated client for the DTZ Flows API
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: jens@apimeister.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Rss2emailFeedFeedIdPostRequest {
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

impl Rss2emailFeedFeedIdPostRequest {
    pub fn new(id: uuid::Uuid, url: String, last_check: String, last_data_found: String, enabled: bool, name: String) -> Rss2emailFeedFeedIdPostRequest {
        Rss2emailFeedFeedIdPostRequest {
            id,
            url,
            last_check,
            last_data_found,
            enabled,
            name,
        }
    }
}


