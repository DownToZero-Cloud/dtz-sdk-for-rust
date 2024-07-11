/*
 * DTZ RSS2Email Api
 *
 * a generated client for the DTZ RSS2Email API
 *
 * The version of the OpenAPI document: 1.0.4
 * Contact: jens@apimeister.com
 * Generated by: https://openapi-generator.tech
 */

#[allow(unused)]
use crate::models;




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiscoverFeedRequest {
    #[serde(rename = "url")]
    pub url: String,
}

impl DiscoverFeedRequest {
    pub fn new(url: String) -> DiscoverFeedRequest {
        DiscoverFeedRequest {
            url,
        }
    }
}


