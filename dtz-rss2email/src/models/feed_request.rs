/*
 * DTZ RSS2Email Api
 *
 * a generated client for the DTZ RSS2Email API
 *
 * Contact: jens@apimeister.com
 * Generated by: https://openapi-generator.tech
 */

#[allow(unused)]
use crate::models;




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FeedRequest {
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl FeedRequest {
    pub fn new(url: String) -> FeedRequest {
        FeedRequest {
            url,
            enabled: None,
        }
    }
}


