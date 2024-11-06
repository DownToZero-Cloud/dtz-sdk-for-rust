/*
 * DTZ RSS2Email Api
 *
 * a generated client for the DTZ RSS2Email API
 *
 * The version of the OpenAPI document: 1.0.6
 * Contact: jens@apimeister.com
 * Generated by: https://openapi-generator.tech
 */

#[allow(unused)]
use crate::models;




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiscoverFeed200Response {
    #[serde(rename = "feedUrl", skip_serializing_if = "Option::is_none")]
    pub feed_url: Option<String>,
    #[serde(rename = "feedType", skip_serializing_if = "Option::is_none")]
    pub feed_type: Option<String>,
}

impl DiscoverFeed200Response {
    pub fn new() -> DiscoverFeed200Response {
        DiscoverFeed200Response {
            feed_url: None,
            feed_type: None,
        }
    }
}


