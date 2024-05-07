/*
 * DTZ RSS2Email Api
 *
 * a generated client for the DTZ RSS2Email API
 *
 * The version of the OpenAPI document: 1.0.1
 * Contact: jens@apimeister.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateFeedRequest {
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "targetEmail", skip_serializing_if = "Option::is_none")]
    pub target_email: Option<String>,
}

impl CreateFeedRequest {
    pub fn new(url: String) -> CreateFeedRequest {
        CreateFeedRequest {
            url,
            target_email: None,
        }
    }
}


