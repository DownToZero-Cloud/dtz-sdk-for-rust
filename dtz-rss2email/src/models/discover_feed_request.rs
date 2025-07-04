/*
 * DTZ RSS2Email Api
 *
 * a generated client for the DTZ RSS2Email API
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

